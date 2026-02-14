use rand::Rng;
#[cfg(feature = "json")]
use serde::Serialize;

use crate::personal_id::checksum::{iso7064_mod11_10, luhn_check, weighted_check};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct VatResult {
    pub code: String,
    pub country_code: String,
    pub country_name: String,
    pub valid: bool,
}

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub country: Option<String>,
}

static COUNTRIES: &[(&str, &str)] = &[
    ("AT", "Austria"),
    ("BE", "Belgium"),
    ("BG", "Bulgaria"),
    ("CY", "Cyprus"),
    ("CZ", "Czech Republic"),
    ("DE", "Germany"),
    ("DK", "Denmark"),
    ("EE", "Estonia"),
    ("EL", "Greece"),
    ("ES", "Spain"),
    ("FI", "Finland"),
    ("FR", "France"),
    ("GB", "United Kingdom"),
    ("HR", "Croatia"),
    ("HU", "Hungary"),
    ("IE", "Ireland"),
    ("IT", "Italy"),
    ("LT", "Lithuania"),
    ("LU", "Luxembourg"),
    ("LV", "Latvia"),
    ("MT", "Malta"),
    ("NL", "Netherlands"),
    ("PL", "Poland"),
    ("PT", "Portugal"),
    ("RO", "Romania"),
    ("SE", "Sweden"),
    ("SI", "Slovenia"),
    ("SK", "Slovakia"),
];

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

    pub fn list_countries(&self) -> &'static [(&'static str, &'static str)] {
        COUNTRIES
    }

    pub fn generate(&self, opts: &GenOptions, rng: &mut impl Rng) -> Option<VatResult> {
        let country = opts
            .country
            .as_deref()
            .unwrap_or_else(|| COUNTRIES[rng.gen_range(0..COUNTRIES.len())].0)
            .to_uppercase();

        // Accept GR as alias for EL
        let country = if country == "GR" {
            "EL".to_string()
        } else {
            country
        };

        let country_name = COUNTRIES
            .iter()
            .find(|(c, _)| *c == country)
            .map(|(_, n)| *n)?;

        let digits = match country.as_str() {
            "AT" => self.generate_at(rng),
            "BE" => self.generate_be(rng),
            "BG" => self.generate_bg(rng),
            "CY" => self.generate_cy(rng),
            "CZ" => self.generate_cz(rng),
            "DE" => self.generate_de(rng),
            "DK" => self.generate_dk(rng),
            "EE" => self.generate_ee(rng),
            "EL" => self.generate_el(rng),
            "ES" => self.generate_es(rng),
            "FI" => self.generate_fi(rng),
            "FR" => self.generate_fr(rng),
            "GB" => self.generate_gb(rng),
            "HR" => self.generate_hr(rng),
            "HU" => self.generate_hu(rng),
            "IE" => self.generate_ie(rng),
            "IT" => self.generate_it(rng),
            "LT" => self.generate_lt(rng),
            "LU" => self.generate_lu(rng),
            "LV" => self.generate_lv(rng),
            "MT" => self.generate_mt(rng),
            "NL" => self.generate_nl(rng),
            "PL" => self.generate_pl(rng),
            "PT" => self.generate_pt(rng),
            "RO" => self.generate_ro(rng),
            "SE" => self.generate_se(rng),
            "SI" => self.generate_si(rng),
            "SK" => self.generate_sk(rng),
            _ => return None,
        };

        let code = format!("{}{}", country, digits);
        Some(VatResult {
            code,
            country_code: country,
            country_name: country_name.to_string(),
            valid: true,
        })
    }

    pub fn validate(&self, code: &str) -> bool {
        let code = code.replace([' ', '-', '.'], "");
        if code.len() < 4 {
            return false;
        }

        // Try to extract country prefix (2 or 3 chars for ATU)
        let (prefix, body) = if let Some(stripped) = code.strip_prefix("ATU") {
            ("AT", stripped)
        } else {
            let prefix = &code[..2];
            let body = &code[2..];
            (prefix, body)
        };

        // Accept GR as alias for EL
        let prefix = if prefix == "GR" { "EL" } else { prefix };

        if !COUNTRIES.iter().any(|(c, _)| *c == prefix) {
            return false;
        }

        match prefix {
            "AT" => self.validate_at(body),
            "BE" => self.validate_be(body),
            "BG" => self.validate_bg(body),
            "CY" => self.validate_cy(body),
            "CZ" => self.validate_cz(body),
            "DE" => self.validate_de(body),
            "DK" => self.validate_dk(body),
            "EE" => self.validate_ee(body),
            "EL" => self.validate_el(body),
            "ES" => self.validate_es(body),
            "FI" => self.validate_fi(body),
            "FR" => self.validate_fr(body),
            "GB" => self.validate_gb(body),
            "HR" => self.validate_hr(body),
            "HU" => self.validate_hu(body),
            "IE" => self.validate_ie(body),
            "IT" => self.validate_it(body),
            "LT" => self.validate_lt(body),
            "LU" => self.validate_lu(body),
            "LV" => self.validate_lv(body),
            "MT" => self.validate_mt(body),
            "NL" => self.validate_nl(body),
            "PL" => self.validate_pl(body),
            "PT" => self.validate_pt(body),
            "RO" => self.validate_ro(body),
            "SE" => self.validate_se(body),
            "SI" => self.validate_si(body),
            "SK" => self.validate_sk(body),
            _ => false,
        }
    }

    // ── AT: ATU + 8 digits, last is Luhn-variant check ──

    fn generate_at(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
        // AT VAT uses a special Luhn variant: weights [1,2,1,2,1,2,1] positions 0-6
        // s_i: if doubled >= 10, subtract 9; sum of s_i mod 10; check = (96 - sum) mod 10
        let mut sum: u32 = 0;
        let weights = [1u32, 2, 1, 2, 1, 2, 1];
        for (d, w) in digits.iter().zip(weights.iter()) {
            let mut v = *d as u32 * w;
            if v >= 10 {
                v -= 9;
            }
            sum += v;
        }
        let check = ((96 - sum) % 10) as u8;
        digits.push(check);
        format!(
            "U{}",
            digits
                .iter()
                .map(|d| (b'0' + d) as char)
                .collect::<String>()
        )
    }

    fn validate_at(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let weights = [1u32, 2, 1, 2, 1, 2, 1];
        let mut sum: u32 = 0;
        for (d, w) in digits[..7].iter().zip(weights.iter()) {
            let mut v = *d as u32 * w;
            if v >= 10 {
                v -= 9;
            }
            sum += v;
        }
        let check = ((96 - sum) % 10) as u8;
        digits[7] == check
    }

    // ── BE: 10 digits, first != 0, last 2 = 97 - (first 8 mod 97) ──

    fn generate_be(&self, rng: &mut impl Rng) -> String {
        let d0 = rng.gen_range(0..2u8); // 0 or 1
        let rest: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
        let num: u64 = std::iter::once(d0)
            .chain(rest.iter().copied())
            .fold(0u64, |acc, d| acc * 10 + d as u64);
        let check = 97 - (num % 97);
        format!(
            "{}{}{:02}",
            d0,
            rest.iter().map(|d| (b'0' + d) as char).collect::<String>(),
            check
        )
    }

    fn validate_be(&self, body: &str) -> bool {
        if body.len() != 10 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let num: u64 = body[..8].parse().unwrap_or(0);
        let check: u64 = body[8..].parse().unwrap_or(0);
        check == 97 - (num % 97)
    }

    // ── BG: 9 or 10 digits, weighted mod 11 ──

    fn generate_bg(&self, rng: &mut impl Rng) -> String {
        // Generate 9-digit variant (company)
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        let weights: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        let mut sum = weighted_check(&digits, weights, 11);
        if sum == 10 {
            let weights2: &[u8] = &[3, 4, 5, 6, 7, 8, 9, 10];
            sum = weighted_check(&digits, weights2, 11);
            if sum == 10 {
                sum = 0;
            }
        }
        digits.push(sum as u8);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_bg(&self, body: &str) -> bool {
        if !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        match body.len() {
            9 => {
                let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
                let weights: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
                let mut sum = weighted_check(&digits[..8], weights, 11);
                if sum == 10 {
                    let weights2: &[u8] = &[3, 4, 5, 6, 7, 8, 9, 10];
                    sum = weighted_check(&digits[..8], weights2, 11);
                    if sum == 10 {
                        sum = 0;
                    }
                }
                digits[8] == sum as u8
            }
            10 => {
                // 10-digit personal: validate as EGN-style
                let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
                let weights: &[u8] = &[2, 4, 8, 5, 10, 9, 7, 3, 6];
                let sum = weighted_check(&digits[..9], weights, 11) % 10;
                digits[9] == sum as u8
            }
            _ => false,
        }
    }

    // ── CY: 8 digits + 1 letter ──

    fn generate_cy(&self, rng: &mut impl Rng) -> String {
        let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        // Odd-position translation table (0-indexed positions 0,2,4,6)
        let odd_vals = [1u32, 0, 5, 7, 9, 13, 15, 17, 19, 21];
        let mut sum: u32 = 0;
        for (i, &d) in digits.iter().enumerate() {
            if i % 2 == 0 {
                // odd position in 1-indexed
                sum += odd_vals[d as usize];
            } else {
                sum += d as u32;
            }
        }
        let check = (sum % 26) as u8;
        let check_letter = (b'A' + check) as char;
        format!(
            "{}{}",
            digits
                .iter()
                .map(|d| (b'0' + d) as char)
                .collect::<String>(),
            check_letter
        )
    }

    fn validate_cy(&self, body: &str) -> bool {
        if body.len() != 9 {
            return false;
        }
        let digit_part = &body[..8];
        let check_char = body.as_bytes()[8];
        if !digit_part.chars().all(|c| c.is_ascii_digit()) || !check_char.is_ascii_uppercase() {
            return false;
        }
        let digits: Vec<u8> = digit_part.bytes().map(|b| b - b'0').collect();
        let odd_vals = [1u32, 0, 5, 7, 9, 13, 15, 17, 19, 21];
        let mut sum: u32 = 0;
        for (i, &d) in digits.iter().enumerate() {
            if i % 2 == 0 {
                sum += odd_vals[d as usize];
            } else {
                sum += d as u32;
            }
        }
        let expected = b'A' + (sum % 26) as u8;
        check_char == expected
    }

    // ── CZ: 8, 9, or 10 digits ──

    fn generate_cz(&self, rng: &mut impl Rng) -> String {
        // Generate 8-digit legal entity: weighted mod 11
        let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
        // First digit: 1-8 (9 is not valid for legal entities)
        digits[0] = rng.gen_range(1..9);
        let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
        let sum = weighted_check(&digits, weights, 11);
        let check = match sum {
            0 => 1,
            1 => 0,
            x => (11 - x) as u8,
        };
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_cz(&self, body: &str) -> bool {
        if !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        match body.len() {
            8 => {
                let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
                let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
                let sum = weighted_check(&digits[..7], weights, 11);
                let expected = match sum {
                    0 => 1,
                    1 => 0,
                    x => (11 - x) as u8,
                };
                digits[7] == expected
            }
            9 | 10 => {
                // Individual: basic format check only (birth date based)
                true
            }
            _ => false,
        }
    }

    // ── DE: 9 digits, ISO 7064 Mod 11,10 ──

    fn generate_de(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        // First digit must not be 0
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        let check = iso7064_mod11_10(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_de(&self, body: &str) -> bool {
        if body.len() != 9 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        let check = iso7064_mod11_10(&digits[..8]);
        digits[8] == check
    }

    // ── DK: 8 digits, weighted mod 11 ──

    fn generate_dk(&self, rng: &mut impl Rng) -> String {
        loop {
            let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
            // First digit non-zero
            if digits[0] == 0 {
                digits[0] = rng.gen_range(1..10);
            }
            let weights: &[u8] = &[2, 7, 6, 5, 4, 3, 2];
            let sum = weighted_check(&digits, weights, 11);
            let check = if sum == 0 { 0 } else { 11 - sum as u8 };
            if check < 10 {
                digits.push(check);
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
            // check == 10 is invalid, retry
        }
    }

    fn validate_dk(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        let weights: &[u8] = &[2, 7, 6, 5, 4, 3, 2, 1];
        let sum = weighted_check(&digits, weights, 11);
        sum == 0
    }

    // ── EE: 9 digits, weighted mod 10 ──

    fn generate_ee(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        // First digit 1
        digits[0] = 1;
        let weights: &[u8] = &[3, 7, 1, 3, 7, 1, 3, 7];
        let sum = weighted_check(&digits, weights, 10);
        let check = if sum == 0 { 0 } else { (10 - sum) as u8 };
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_ee(&self, body: &str) -> bool {
        if body.len() != 9 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let weights: &[u8] = &[3, 7, 1, 3, 7, 1, 3, 7];
        let sum = weighted_check(&digits[..8], weights, 10);
        let expected = if sum == 0 { 0 } else { (10 - sum) as u8 };
        digits[8] == expected
    }

    // ── EL (Greece): 9 digits, iterative ×2 mod 11 ──

    fn generate_el(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        // First digit non-zero
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        // Check: multiply from left, each step: prev*2 + digit, mod 11, final mod 10
        let mut sum: u32 = 0;
        for &d in &digits {
            sum = (sum * 2 + d as u32) % 11;
        }
        let check = (sum * 2 % 11 % 10) as u8;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_el(&self, body: &str) -> bool {
        if body.len() != 9 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        let mut sum: u32 = 0;
        for &d in &digits[..8] {
            sum = (sum * 2 + d as u32) % 11;
        }
        let check = (sum * 2 % 11 % 10) as u8;
        digits[8] == check
    }

    // ── ES: 1 letter/digit + 7 digits + 1 letter/digit ──

    fn generate_es(&self, rng: &mut impl Rng) -> String {
        // Generate NIF style: letter + 7 digits + letter
        static NIF_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";
        let num: u32 = rng.gen_range(0..100_000_000);
        let check = NIF_LETTERS[(num % 23) as usize] as char;
        format!("{:08}{}", num, check)
    }

    fn validate_es(&self, body: &str) -> bool {
        if body.len() != 9 {
            return false;
        }
        static NIF_LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";
        let bytes = body.as_bytes();

        // Type 1: 8 digits + letter (DNI/NIF)
        if bytes[..8].iter().all(|b| b.is_ascii_digit()) && bytes[8].is_ascii_uppercase() {
            let num: u32 = body[..8].parse().unwrap_or(0);
            return bytes[8] == NIF_LETTERS[(num % 23) as usize];
        }

        // Type 2: letter + 7 digits + letter (NIE: X, Y, Z prefix)
        if matches!(bytes[0], b'X' | b'Y' | b'Z')
            && bytes[1..8].iter().all(|b| b.is_ascii_digit())
            && bytes[8].is_ascii_uppercase()
        {
            let prefix_val = match bytes[0] {
                b'X' => 0u32,
                b'Y' => 1,
                b'Z' => 2,
                _ => return false,
            };
            let num: u32 = body[1..8].parse().unwrap_or(0) + prefix_val * 10_000_000;
            return bytes[8] == NIF_LETTERS[(num % 23) as usize];
        }

        // Type 3: CIF (letter + 7 digits + letter/digit)
        if bytes[0].is_ascii_uppercase()
            && bytes[1..8].iter().all(|b| b.is_ascii_digit())
            && (bytes[8].is_ascii_uppercase() || bytes[8].is_ascii_digit())
        {
            let digits: Vec<u8> = body[1..8].bytes().map(|b| b - b'0').collect();
            let mut even_sum: u32 = 0;
            let mut odd_sum: u32 = 0;
            for (i, &d) in digits.iter().enumerate() {
                if i % 2 == 0 {
                    let doubled = d as u32 * 2;
                    odd_sum += doubled / 10 + doubled % 10;
                } else {
                    even_sum += d as u32;
                }
            }
            let total = even_sum + odd_sum;
            let check_digit = ((10 - (total % 10)) % 10) as u8;

            // Some prefixes use digit, others use letter
            if bytes[8].is_ascii_digit() {
                return bytes[8] - b'0' == check_digit;
            } else {
                return bytes[8] == b'A' + check_digit;
            }
        }

        false
    }

    // ── FI: 8 digits, weighted mod 11 ──

    fn generate_fi(&self, rng: &mut impl Rng) -> String {
        loop {
            let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
            if digits[0] == 0 {
                digits[0] = rng.gen_range(1..10);
            }
            let weights: &[u8] = &[7, 9, 10, 5, 8, 4, 2];
            let sum = weighted_check(&digits, weights, 11);
            let check = if sum == 0 { 0 } else { 11 - sum as u8 };
            if check < 10 {
                digits.push(check);
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }

    fn validate_fi(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let weights: &[u8] = &[7, 9, 10, 5, 8, 4, 2];
        let sum = weighted_check(&digits[..7], weights, 11);
        let expected = if sum == 0 { 0 } else { (11 - sum) as u8 };
        expected < 10 && digits[7] == expected
    }

    // ── FR: 2 check chars (digits or letters) + 9 digit SIREN ──

    fn generate_fr(&self, rng: &mut impl Rng) -> String {
        // SIREN is 9 digits with the last being a Luhn check digit
        let mut siren_digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        if siren_digits[0] == 0 {
            siren_digits[0] = rng.gen_range(1..10);
        }
        let check = luhn_check(&siren_digits);
        siren_digits.push(check);
        let siren: u64 = siren_digits
            .iter()
            .fold(0u64, |acc, &d| acc * 10 + d as u64);
        // FR VAT key = (siren * 100 + 12) % 97 (equivalent to (12 + 3 * (siren % 97)) % 97)
        let key = (12 + 3 * (siren % 97)) % 97;
        format!("{:02}{:09}", key, siren)
    }

    fn validate_fr(&self, body: &str) -> bool {
        if body.len() != 11 {
            return false;
        }
        // Check digits can be alphanumeric (O, I excluded historically but we accept digits)
        let key_str = &body[..2];
        let siren_str = &body[2..];
        if !siren_str.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        // Numeric key variant
        if key_str.chars().all(|c| c.is_ascii_digit()) {
            let key: u64 = key_str.parse().unwrap_or(0);
            let siren: u64 = siren_str.parse().unwrap_or(0);
            let expected = (12 + 3 * (siren % 97)) % 97;
            return key == expected;
        }
        // Alphanumeric key variant: full number mod 97
        // Convert letters: A=0..Z=25 mapped to numeric
        // For now accept only numeric keys in validation
        false
    }

    // ── GB: 9 digits, weighted mod 97 ──

    fn generate_gb(&self, rng: &mut impl Rng) -> String {
        loop {
            let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
            // First 2 digits: 01-97 (not 00)
            if digits[0] == 0 && digits[1] == 0 {
                digits[0] = rng.gen_range(0..10);
                digits[1] = rng.gen_range(1..10);
            }
            let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum();

            // Try both variants: check = sum mod 97, or check = (sum + 55) mod 97
            for offset in [0u32, 55] {
                let rem = (sum + offset) % 97;
                if rem == 0 {
                    digits.push(0);
                    digits.push(0);
                    return digits.iter().map(|d| (b'0' + d) as char).collect();
                }
                let check = 97 - rem;
                if check < 100 {
                    digits.push((check / 10) as u8);
                    digits.push((check % 10) as u8);
                    return digits.iter().map(|d| (b'0' + d) as char).collect();
                }
            }
        }
    }

    fn validate_gb(&self, body: &str) -> bool {
        if body.len() != 9 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let sum: u32 = digits[..7]
            .iter()
            .zip([8u32, 7, 6, 5, 4, 3, 2].iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        let check_val: u32 = digits[7] as u32 * 10 + digits[8] as u32;
        let total = sum + check_val;
        total.is_multiple_of(97) || (total + 55).is_multiple_of(97)
    }

    // ── HR: 11 digits, ISO 7064 Mod 11,10 ──

    fn generate_hr(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..10)).collect();
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        let check = iso7064_mod11_10(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_hr(&self, body: &str) -> bool {
        if body.len() != 11 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let check = iso7064_mod11_10(&digits[..10]);
        digits[10] == check
    }

    // ── HU: 8 digits, weighted mod 10 ──

    fn generate_hu(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
        // First digit: 1-9 (taxpayer type)
        digits[0] = rng.gen_range(1..10);
        let weights: &[u8] = &[9, 7, 3, 1, 9, 7, 3];
        let sum = weighted_check(&digits, weights, 10);
        let check = if sum == 0 { 0 } else { (10 - sum) as u8 };
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_hu(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        let weights: &[u8] = &[9, 7, 3, 1, 9, 7, 3];
        let sum = weighted_check(&digits[..7], weights, 10);
        let expected = if sum == 0 { 0 } else { (10 - sum) as u8 };
        digits[7] == expected
    }

    // ── IE: 7 digits + 1-2 letters ──

    fn generate_ie(&self, rng: &mut impl Rng) -> String {
        // New style: 7 digits + 1 letter + optional 'W' for married women
        let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
        let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
        let sum = weighted_check(&digits, weights, 23);
        let check = if sum == 0 { b'W' } else { b'A' + sum as u8 - 1 };
        format!(
            "{}{}",
            digits
                .iter()
                .map(|d| (b'0' + d) as char)
                .collect::<String>(),
            check as char
        )
    }

    fn validate_ie(&self, body: &str) -> bool {
        let len = body.len();
        if len != 8 && len != 9 {
            return false;
        }
        let digit_part = &body[..7];
        if !digit_part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let check_char = body.as_bytes()[7];
        if !check_char.is_ascii_uppercase() {
            return false;
        }
        let digits: Vec<u8> = digit_part.bytes().map(|b| b - b'0').collect();

        // New format (8 chars): multiply_factor for second letter
        let extra_weight: u32 = if len == 9 {
            let second = body.as_bytes()[8];
            if !second.is_ascii_uppercase() {
                return false;
            }
            // The second letter acts as a weight-9 position
            (second - b'A' + 1) as u32 * 9
        } else {
            0
        };

        let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
        let sum = weighted_check(&digits, weights, 23);
        // Add extra_weight contribution
        let total = (sum + extra_weight % 23) % 23;
        let expected = if total == 0 {
            b'W'
        } else {
            b'A' + total as u8 - 1
        };
        check_char == expected
    }

    // ── IT: 11 digits, Luhn ──

    fn generate_it(&self, rng: &mut impl Rng) -> String {
        // First 7 digits: sequential number (non-zero start)
        let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        // Digits 8-10: province code (001-100, 120, 121, 888, 999)
        let province = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68,
            69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
            91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 120, 121, 888, 999,
        ];
        let prov = province[rng.gen_range(0..province.len())];
        digits.push((prov / 100) as u8);
        digits.push((prov / 10 % 10) as u8);
        digits.push((prov % 10) as u8);
        let check = luhn_check(&digits);
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_it(&self, body: &str) -> bool {
        if body.len() != 11 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let check = luhn_check(&digits[..10]);
        digits[10] == check
    }

    // ── LT: 9 or 12 digits ──

    fn generate_lt(&self, rng: &mut impl Rng) -> String {
        // Generate 9-digit variant
        let mut digits: Vec<u8> = vec![0; 9];
        for d in digits[..7].iter_mut() {
            *d = rng.gen_range(0..10);
        }
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        // Position 8 (0-indexed 7) must be 1 for 9-digit
        digits[7] = 1;
        // Check digit
        let weights1: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
        let sum: u32 = digits[..8]
            .iter()
            .zip(weights1.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let mut check = sum % 11;
        if check == 10 {
            let weights2: &[u8] = &[3, 4, 5, 6, 7, 8, 9, 1];
            let sum2: u32 = digits[..8]
                .iter()
                .zip(weights2.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum();
            check = sum2 % 11;
            if check == 10 {
                check = 0;
            }
        }
        digits[8] = check as u8;
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_lt(&self, body: &str) -> bool {
        if !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        match body.len() {
            9 => {
                let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
                if digits[7] != 1 {
                    return false;
                }
                let weights1: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
                let sum: u32 = digits[..8]
                    .iter()
                    .zip(weights1.iter())
                    .map(|(&d, &w)| d as u32 * w as u32)
                    .sum();
                let mut check = sum % 11;
                if check == 10 {
                    let weights2: &[u8] = &[3, 4, 5, 6, 7, 8, 9, 1];
                    let sum2: u32 = digits[..8]
                        .iter()
                        .zip(weights2.iter())
                        .map(|(&d, &w)| d as u32 * w as u32)
                        .sum();
                    check = sum2 % 11;
                    if check == 10 {
                        check = 0;
                    }
                }
                digits[8] == check as u8
            }
            12 => {
                let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
                if digits[10] != 1 {
                    return false;
                }
                let weights1: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2];
                let sum: u32 = digits[..11]
                    .iter()
                    .zip(weights1.iter())
                    .map(|(&d, &w)| d as u32 * w as u32)
                    .sum();
                let mut check = sum % 11;
                if check == 10 {
                    let weights2: &[u8] = &[3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4];
                    let sum2: u32 = digits[..11]
                        .iter()
                        .zip(weights2.iter())
                        .map(|(&d, &w)| d as u32 * w as u32)
                        .sum();
                    check = sum2 % 11;
                    if check == 10 {
                        check = 0;
                    }
                }
                digits[11] == check as u8
            }
            _ => false,
        }
    }

    // ── LU: 8 digits, last 2 = first 6 mod 89 + 1... actually simpler: mod 89 ──

    fn generate_lu(&self, rng: &mut impl Rng) -> String {
        let first6: u32 = rng.gen_range(100_000..1_000_000);
        let check = first6 % 89;
        format!("{:06}{:02}", first6, check)
    }

    fn validate_lu(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let first6: u32 = body[..6].parse().unwrap_or(0);
        let check: u32 = body[6..].parse().unwrap_or(0);
        check == first6 % 89
    }

    // ── LV: 11 digits ──

    fn generate_lv(&self, rng: &mut impl Rng) -> String {
        // Legal entity: starts with 4, 5, 6, or 9
        // Checksum: sum of all 11 digits * weights (9,1,4,8,3,10,2,5,7,6,1) mod 11 = 3
        let first = [4u8, 5, 6, 9][rng.gen_range(0..4)];
        loop {
            let mut digits: Vec<u8> = vec![first];
            for _ in 0..10 {
                digits.push(rng.gen_range(0..10));
            }
            let weights: &[u8] = &[9, 1, 4, 8, 3, 10, 2, 5, 7, 6, 1];
            let sum = weighted_check(&digits, weights, 11);
            if sum == 3 {
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }

    fn validate_lv(&self, body: &str) -> bool {
        if body.len() != 11 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        // Personal (starts with 0-3): format check only
        if digits[0] <= 3 {
            return true;
        }
        // Legal entity: sum of all 11 * weights mod 11 = 3
        let weights: &[u8] = &[9, 1, 4, 8, 3, 10, 2, 5, 7, 6, 1];
        let sum = weighted_check(&digits, weights, 11);
        sum == 3
    }

    // ── MT: 8 digits, weighted mod 37 ──

    fn generate_mt(&self, rng: &mut impl Rng) -> String {
        loop {
            let mut digits: Vec<u8> = (0..6).map(|_| rng.gen_range(0..10)).collect();
            if digits[0] == 0 {
                digits[0] = rng.gen_range(1..10);
            }
            let weights: &[u8] = &[3, 4, 6, 7, 8, 9];
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum();
            let check = 37 - (sum % 37);
            if check < 100 {
                digits.push((check / 10) as u8);
                digits.push((check % 10) as u8);
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }

    fn validate_mt(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let weights: &[u8] = &[3, 4, 6, 7, 8, 9];
        let sum: u32 = digits[..6]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = 37 - (sum % 37);
        let actual_check = digits[6] as u32 * 10 + digits[7] as u32;
        actual_check == check
    }

    // ── NL: 9 digits + B + 2 digits (mod 97) ──

    fn generate_nl(&self, rng: &mut impl Rng) -> String {
        // Full string: "NL" + 9digits + "B" + 2check
        // Numeric: N=23,L=21 -> "2321" + 9digits + "11" (B) + 2check
        // Must satisfy mod 97 = 1, and check > 0
        loop {
            let base: u64 = rng.gen_range(100_000_000..1_000_000_000);
            let placeholder = format!("2321{:09}1100", base);
            let rem = mod97_str(&placeholder);
            let check = (98 - rem) % 97;
            if check > 0 {
                return format!("{:09}B{:02}", base, check);
            }
        }
    }

    fn validate_nl(&self, body: &str) -> bool {
        if body.len() != 12 {
            return false;
        }
        if &body[9..10] != "B" {
            return false;
        }
        let digit_part1 = &body[..9];
        let digit_part2 = &body[10..];
        if !digit_part1.chars().all(|c| c.is_ascii_digit())
            || !digit_part2.chars().all(|c| c.is_ascii_digit())
        {
            return false;
        }
        // Full numeric: "NL" + body -> N=23, L=21, B=11
        let numeric = format!("2321{}11{}", digit_part1, digit_part2);
        mod97_str(&numeric) == 1
    }

    // ── PL: 10 digits, weighted mod 11 ──

    fn generate_pl(&self, rng: &mut impl Rng) -> String {
        loop {
            let mut digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..10)).collect();
            if digits[0] == 0 {
                digits[0] = rng.gen_range(1..10);
            }
            let weights: &[u8] = &[6, 5, 7, 2, 3, 4, 5, 6, 7];
            let sum = weighted_check(&digits, weights, 11);
            if sum < 10 {
                digits.push(sum as u8);
                return digits.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }

    fn validate_pl(&self, body: &str) -> bool {
        if body.len() != 10 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let weights: &[u8] = &[6, 5, 7, 2, 3, 4, 5, 6, 7];
        let sum = weighted_check(&digits[..9], weights, 11);
        sum < 10 && digits[9] == sum as u8
    }

    // ── PT: 9 digits, weighted mod 11 ──

    fn generate_pt(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..10)).collect();
        // First digit: 1-9
        digits[0] = rng.gen_range(1..10);
        let weights: &[u8] = &[9, 8, 7, 6, 5, 4, 3, 2];
        let sum = weighted_check(&digits, weights, 11);
        let check = if sum <= 1 { 0 } else { (11 - sum) as u8 };
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_pt(&self, body: &str) -> bool {
        if body.len() != 9 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let weights: &[u8] = &[9, 8, 7, 6, 5, 4, 3, 2];
        let sum = weighted_check(&digits[..8], weights, 11);
        let expected = if sum <= 1 { 0 } else { (11 - sum) as u8 };
        digits[8] == expected
    }

    // ── RO: 2-10 digits ──

    fn generate_ro(&self, rng: &mut impl Rng) -> String {
        // Generate a valid 2-10 digit number
        let len = rng.gen_range(2..=10);
        let mut digits: Vec<u8> = (0..len - 1).map(|_| rng.gen_range(0..10)).collect();
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        // Weights from the right: 7,5,3,2,1,7,5,3,2 (for up to 10 digits)
        let all_weights: &[u8] = &[7, 5, 3, 2, 1, 7, 5, 3, 2];
        let w_start = 9usize.saturating_sub(digits.len());
        let weights = &all_weights[w_start..];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = (sum * 10 % 11 % 10) as u8;
        digits.push(check);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_ro(&self, body: &str) -> bool {
        let len = body.len();
        if !(2..=10).contains(&len) || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        let all_weights: &[u8] = &[7, 5, 3, 2, 1, 7, 5, 3, 2];
        let payload = &digits[..len - 1];
        let w_start = 9usize.saturating_sub(payload.len());
        let weights = &all_weights[w_start..];
        let sum: u32 = payload
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let expected = (sum * 10 % 11 % 10) as u8;
        digits[len - 1] == expected
    }

    // ── SE: 12 digits, Luhn on first 10 + "01" suffix ──

    fn generate_se(&self, rng: &mut impl Rng) -> String {
        let mut digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..10)).collect();
        if digits[0] == 0 {
            digits[0] = rng.gen_range(1..10);
        }
        let check = luhn_check(&digits);
        digits.push(check);
        // Append "01"
        digits.push(0);
        digits.push(1);
        digits.iter().map(|d| (b'0' + d) as char).collect()
    }

    fn validate_se(&self, body: &str) -> bool {
        if body.len() != 12 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        // Last 2 digits must be "01"
        if &body[10..] != "01" {
            return false;
        }
        let digits: Vec<u8> = body[..10].bytes().map(|b| b - b'0').collect();
        let check = luhn_check(&digits[..9]);
        digits[9] == check
    }

    // ── SI: 8 digits, weighted mod 11 ──

    fn generate_si(&self, rng: &mut impl Rng) -> String {
        loop {
            let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..10)).collect();
            if digits[0] == 0 {
                digits[0] = rng.gen_range(1..10);
            }
            let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
            let sum = weighted_check(&digits, weights, 11);
            let check = if sum == 0 || sum == 1 {
                // Invalid, retry. SI check digit cannot be 0 or 1 (would mean mod result is 0 or 10)
                continue;
            } else {
                (11 - sum) as u8
            };
            if check >= 10 {
                continue;
            }
            digits.push(check);
            return digits.iter().map(|d| (b'0' + d) as char).collect();
        }
    }

    fn validate_si(&self, body: &str) -> bool {
        if body.len() != 8 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        let weights: &[u8] = &[8, 7, 6, 5, 4, 3, 2];
        let sum = weighted_check(&digits[..7], weights, 11);
        if sum <= 1 {
            return false;
        }
        let expected = (11 - sum) as u8;
        expected < 10 && digits[7] == expected
    }

    // ── SK: 10 digits, divisible by 11 ──

    fn generate_sk(&self, rng: &mut impl Rng) -> String {
        // Format: 10 digits, first digit non-zero, 3rd digit in {2,3,4,7,8,9}, divisible by 11
        let valid_d2 = [2u8, 3, 4, 7, 8, 9];
        loop {
            let d0 = rng.gen_range(1..10u8);
            let d1 = rng.gen_range(0..10u8);
            let d2 = valid_d2[rng.gen_range(0..valid_d2.len())];
            let rest: u64 = rng.gen_range(0..10_000_000);
            let num =
                d0 as u64 * 1_000_000_000 + d1 as u64 * 100_000_000 + d2 as u64 * 10_000_000 + rest;
            if num.is_multiple_of(11) {
                return format!("{:010}", num);
            }
        }
    }

    fn validate_sk(&self, body: &str) -> bool {
        if body.len() != 10 || !body.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        if !matches!(digits[2], 2 | 3 | 4 | 7 | 8 | 9) {
            return false;
        }
        let num: u64 = body.parse().unwrap_or(1);
        num.is_multiple_of(11)
    }
}

fn mod97_str(s: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in s.chars() {
        remainder = remainder * 10 + ch.to_digit(10).unwrap_or(0) as u64;
        remainder %= 97;
    }
    remainder
}
