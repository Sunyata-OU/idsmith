use rand::Rng;

use super::IdResult;

fn compute_check(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| d as u32 * (13 - i as u32))
        .sum();
    ((11 - s % 11) % 10) as u8
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut digits: Vec<u8> = Vec::with_capacity(13);
    digits.push(rng.gen_range(1..=8)); // Type digit 1-8
    for _ in 0..11 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = compute_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    if digits[0] == 0 || digits[0] == 9 {
        return false;
    }
    compute_check(&digits[..12]) == digits[12]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: if clean.len() == 13 {
            format!(
                "{}-{}-{}-{}-{}",
                &clean[0..1],
                &clean[1..5],
                &clean[5..10],
                &clean[10..12],
                &clean[12..13]
            )
        } else {
            clean
        },
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
