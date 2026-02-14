use rand::Rng;

use super::checksum;
use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let first: u8 = if rng.gen_bool(0.5) { 1 } else { 2 };
    let mut digits: Vec<u8> = vec![first];
    for _ in 0..8 {
        digits.push(rng.gen_range(0..=9u8));
    }
    let check = checksum::luhn_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 {
        return false;
    }
    if !clean.starts_with('1') && !clean.starts_with('2') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    checksum::luhn_check(&digits[..9]) == digits[9]
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        country_code: "".to_string(),
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
