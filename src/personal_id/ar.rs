use rand::Rng;

use super::date::Gender;
use super::IdResult;

static WEIGHTS: [u32; 10] = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

fn compute_check(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r = 11 - (s % 11);
    match r {
        11 => 0,
        10 => 9, // Adjusted: prefix changes when remainder is 10
        v => v as u8,
    }
}

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let prefix: u8 = match gender {
        Gender::Male => 20,
        Gender::Female => 27,
    };
    let body: u64 = rng.gen_range(10_000_000..=99_999_999);
    let body_str = format!("{:02}{:08}", prefix, body);
    let digits: Vec<u8> = body_str.bytes().map(|b| b - b'0').collect();
    let check = compute_check(&digits);
    format!("{}{}", body_str, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 {
        return false;
    }
    let prefix: u8 = clean[0..2].parse().unwrap_or(0);
    if !matches!(prefix, 20 | 23 | 24 | 25 | 26 | 27 | 30 | 33 | 34) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    compute_check(&digits[..10]) == digits[10]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let gender = if clean.len() == 11 {
        match &clean[0..2] {
            "20" => Some("male".to_string()),
            "27" => Some("female".to_string()),
            _ => None,
        }
    } else {
        None
    };
    IdResult {
        code: if clean.len() == 11 {
            format!("{}-{}-{}", &clean[0..2], &clean[2..10], &clean[10..11])
        } else {
            clean
        },
        gender,
        dob: None,
        valid: validate(code),
    }
}
