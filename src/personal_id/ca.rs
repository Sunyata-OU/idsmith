use rand::Rng;

use super::checksum;
use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = Vec::with_capacity(9);
    // First digit: 1-7 or 9 (0 and 8 are not assigned)
    let first = loop {
        let d = rng.gen_range(1..=9u8);
        if d != 8 { break d; }
    };
    digits.push(first);
    for _ in 0..7 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = checksum::luhn_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != ' ').collect();
    if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if clean.starts_with('0') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    checksum::luhn_check(&digits[..8]) == digits[8]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != ' ').collect();
    IdResult {
        code: if clean.len() == 9 {
            format!("{} {} {}", &clean[0..3], &clean[3..6], &clean[6..9])
        } else {
            clean
        },
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
