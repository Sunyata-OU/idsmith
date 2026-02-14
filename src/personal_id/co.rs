use rand::Rng;

use super::IdResult;

static WEIGHTS: [u32; 9] = [41, 37, 29, 23, 19, 17, 13, 7, 3];

fn compute_check(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r = s % 11;
    if r >= 2 { (11 - r) as u8 } else { r as u8 }
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = Vec::with_capacity(10);
    digits.push(rng.gen_range(1..=9));
    for _ in 0..8 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = compute_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    compute_check(&digits[..9]) == digits[9]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: clean,
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
