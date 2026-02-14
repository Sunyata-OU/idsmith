use rand::Rng;

use super::checksum;
use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9u8)).collect();
    let check = checksum::luhn_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    // Pad to 9 digits
    if clean.is_empty() || clean.len() > 9 {
        return false;
    }
    let padded = format!("{:0>9}", clean);
    let digits: Vec<u8> = padded.bytes().map(|b| b - b'0').collect();
    checksum::luhn_check(&digits[..8]) == digits[8]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: format!("{:0>9}", clean),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
