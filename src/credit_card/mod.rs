use crate::bank_account::checksum::luhn_check_digit;
use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct CardResult {
    pub brand: String,
    pub number: String,
    pub formatted: String,
    pub cvv: String,
    pub expiry: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub brand: Option<String>,
    /// Current year (last two digits, e.g. 26 for 2026).
    /// When `None`, the year is derived from `std::time::SystemTime`.
    /// Set this explicitly when targeting WASM where `SystemTime` is unavailable.
    pub current_year: Option<u16>,
}

pub struct Registry;

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> Option<CardResult> {
        let brand_name = match opts.brand.as_deref() {
            Some(b) => b.to_lowercase(),
            None => {
                let brands = ["visa", "mastercard", "amex", "discover", "jcb", "diners"];
                brands[rng.gen_range(0..brands.len())].to_string()
            }
        };

        let (mut digits, len) = match brand_name.as_str() {
            "visa" => (vec![4], 16),
            "mastercard" => {
                if rng.gen_bool(0.8) {
                    (vec![5, rng.gen_range(1..=5)], 16)
                } else {
                    let p = rng.gen_range(2221..=2720);
                    (p.to_string().bytes().map(|b| b - b'0').collect(), 16)
                }
            }
            "amex" => (vec![3, if rng.gen_bool(0.5) { 4 } else { 7 }], 15),
            "discover" => {
                let r = rng.gen_range(0..3);
                match r {
                    0 => (vec![6, 0, 1, 1], 16),
                    1 => (vec![6, 5], 16),
                    _ => (vec![6, 4, rng.gen_range(4..=9)], 16),
                }
            }
            "jcb" => {
                let p = rng.gen_range(3528..=3589);
                (p.to_string().bytes().map(|b| b - b'0').collect(), 16)
            }
            "diners" => {
                if rng.gen_bool(0.5) {
                    (vec![3, 0, rng.gen_range(0..=5)], 14)
                } else {
                    (vec![3, if rng.gen_bool(0.5) { 6 } else { 8 }], 14)
                }
            }
            _ => return None,
        };

        while digits.len() < len - 1 {
            digits.push(rng.gen_range(0..=9));
        }

        let check = luhn_check_digit(&digits);
        digits.push(check);

        let number: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        let formatted = self.format(&brand_name, &number);

        let cvv_len = if brand_name == "amex" { 4 } else { 3 };
        let cvv: String = (0..cvv_len)
            .map(|_| (b'0' + rng.gen_range(0..=9)) as char)
            .collect();

        let current_year = match opts.current_year {
            Some(y) => y,
            None => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                ((1970 + now / 31_536_000) % 100) as u16
            }
        };
        let year = rng.gen_range(current_year..=current_year + 5);
        let month = rng.gen_range(1..=12u8);
        let expiry = format!("{:02}/{:02}", month, year);

        Some(CardResult {
            brand: brand_name.to_uppercase(),
            number,
            formatted,
            cvv,
            expiry,
            valid: true,
        })
    }

    pub fn validate(&self, number: &str) -> bool {
        let clean: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
        if clean.len() < 13 || clean.len() > 19 {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let payload = &digits[..digits.len() - 1];
        let check = digits[digits.len() - 1];
        luhn_check_digit(payload) == check
    }

    pub fn format(&self, brand: &str, number: &str) -> String {
        match brand.to_lowercase().as_str() {
            "amex" if number.len() == 15 => {
                format!("{} {} {}", &number[0..4], &number[4..10], &number[10..15])
            }
            "diners" if number.len() == 14 => {
                format!("{} {} {}", &number[0..4], &number[4..10], &number[10..14])
            }
            _ => number
                .as_bytes()
                .chunks(4)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    pub fn list_brands(&self) -> Vec<&'static str> {
        vec!["Visa", "Mastercard", "Amex", "Discover", "JCB", "Diners"]
    }
}
