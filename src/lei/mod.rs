use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct LeiResult {
    pub code: String,
    pub lou: String,
    pub country_code: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub country: Option<String>,
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

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> LeiResult {
        let lou: String = (0..4)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'A' + idx - 10) as char
                }
            })
            .collect();

        let country = opts
            .country
            .as_deref()
            .unwrap_or_else(|| {
                let countries = [
                    "US", "GB", "DE", "FR", "IT", "ES", "NL", "CH", "JP", "AU", "CA", "SG", "HK",
                    "IN", "CN",
                ];
                countries[rng.gen_range(0..countries.len())]
            })
            .to_uppercase();

        let entity_part: String = (0..12)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    (b'0' + idx) as char
                } else {
                    (b'A' + idx - 10) as char
                }
            })
            .collect();

        let base = format!("{}{}{}", lou, country, entity_part);

        let numeric = alpha_to_digits(&base);
        let remainder = mod97(&format!("{}00", numeric));
        let check = 98 - remainder;
        let code = format!("{}{:02}", base, check);

        LeiResult {
            lou: code[..4].to_string(),
            country_code: code[4..6].to_string(),
            code,
            valid: true,
        }
    }

    pub fn validate(&self, code: &str) -> bool {
        if code.len() != 20 {
            return false;
        }
        if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
        let numeric = alpha_to_digits(code);
        mod97(&numeric) == 1
    }
}

fn alpha_to_digits(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if c.is_ascii_digit() {
            out.push(c);
        } else {
            let val = c.to_ascii_uppercase() as u32 - b'A' as u32 + 10;
            out.push_str(&val.to_string());
        }
    }
    out
}

fn mod97(s: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in s.chars() {
        remainder = remainder * 10 + ch.to_digit(10).unwrap_or(0) as u64;
        remainder %= 97;
    }
    remainder
}
