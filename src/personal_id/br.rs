use rand::Rng;

use super::IdResult;

fn cpf_check1(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| d as u32 * (10 - i as u32))
        .sum();
    let r = s % 11;
    if r < 2 { 0 } else { (11 - r) as u8 }
}

fn cpf_check2(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| d as u32 * (11 - i as u32))
        .sum();
    let r = s % 11;
    if r < 2 { 0 } else { (11 - r) as u8 }
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9u8)).collect();
    // Avoid all-same-digit numbers
    if digits.iter().all(|&d| d == digits[0]) {
        digits[8] = (digits[8] + 1) % 10;
    }
    let c1 = cpf_check1(&digits);
    digits.push(c1);
    let c2 = cpf_check2(&digits);
    digits.push(c2);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    if digits.iter().all(|&d| d == digits[0]) {
        return false;
    }
    cpf_check1(&digits[..9]) == digits[9] && cpf_check2(&digits[..10]) == digits[10]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: if clean.len() == 11 {
            format!(
                "{}.{}.{}-{}",
                &clean[0..3],
                &clean[3..6],
                &clean[6..9],
                &clean[9..11]
            )
        } else {
            clean
        },
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
