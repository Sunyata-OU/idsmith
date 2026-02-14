use rand::Rng;

use super::checksum;
use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = Vec::with_capacity(12);
    digits.push(rng.gen_range(2..=9)); // First digit: 2-9
    for _ in 0..10 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = checksum::verhoeff_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 12 {
        return false;
    }
    if clean.starts_with('0') || clean.starts_with('1') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    checksum::verhoeff_validate(&digits)
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: if clean.len() == 12 {
            format!("{} {} {}", &clean[0..4], &clean[4..8], &clean[8..12])
        } else {
            clean
        },
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
