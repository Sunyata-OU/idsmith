use super::IdResult;
use crate::bank_account::checksum::luhn_check_digit;
use rand::Rng;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    // Algeria NIF: 15 digits
    // Digits 1-2: Wilaya (01-58)
    // Digits 3-14: Random
    // Digit 15: Luhn checksum (common in many NIF implementations)
    let wilaya = rng.gen_range(1..=58);
    let mut digits: Vec<u8> = vec![(wilaya / 10) as u8, (wilaya % 10) as u8];
    for _ in 0..12 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = luhn_check_digit(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    if code.len() != 15 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let wilaya = digits[0] * 10 + digits[1];
    if wilaya == 0 || wilaya > 58 {
        return false;
    }
    luhn_check_digit(&digits[..14]) == digits[14]
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        country_code: "DZ".to_string(),
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
