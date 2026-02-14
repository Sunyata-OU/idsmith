use rand::Rng;

use super::IdResult;

static WEIGHTS: [u32; 11] = [6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

fn compute_check(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r = s % 11;
    let check = if r <= 1 { 0 } else { 11 - r };
    check as u8
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = (0..11).map(|_| rng.gen_range(0..=9u8)).collect();
    let check = compute_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 12 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    compute_check(&digits[..11]) == digits[11]
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
