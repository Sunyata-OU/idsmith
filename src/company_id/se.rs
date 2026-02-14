use crate::bank_account::checksum::luhn_check_digit;
use rand::Rng;

/// Orgnr (Swedish company number) - 10-digit Luhn.
/// Output as SE{orgnr}01 for VAT format.
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = vec![rng.gen_range(1..=9)];
        for _ in 1..9 {
            digits.push(rng.gen_range(0..=9));
        }
        // 3rd pair (digits[4..6] as number) must be >= 20 for org numbers
        let mid = digits[4] * 10 + digits[5];
        if mid < 20 {
            continue;
        }
        let check = luhn_check_digit(&digits);
        digits.push(check);
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("SE{}01", s);
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("SE") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 12 || !clean.ends_with("01") {
        return false;
    }
    let orgnr = &clean[..10];
    if !orgnr.chars().all(|c| c.is_ascii_digit()) || orgnr.starts_with('0') {
        return false;
    }
    let mid = orgnr[4..6].parse::<u8>().unwrap_or(0);
    if mid < 20 {
        return false;
    }
    let digits: Vec<u8> = orgnr.bytes().map(|b| b - b'0').collect();
    luhn_check_digit(&digits[..9]) == digits[9]
}
