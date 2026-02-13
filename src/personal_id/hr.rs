use rand::Rng;

use super::checksum::iso7064_mod11_10;
use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..=9u8)).collect();
    let check = iso7064_mod11_10(&digits);
    let code: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}", code, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code[..10].bytes().map(|b| b - b'0').collect();
    iso7064_mod11_10(&digits) == code.as_bytes()[10] - b'0'
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
