use rand::Rng;

use super::checksum::weighted_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W1: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 1];
const W2: &[u8] = &[3, 4, 5, 6, 7, 8, 9, 1, 2, 3];

fn check_digit(digits: &[u8]) -> u8 {
    let r = weighted_check(digits, W1, 11);
    if r < 10 {
        return r as u8;
    }
    let r = weighted_check(digits, W2, 11);
    if r < 10 {
        r as u8
    } else {
        0
    }
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let century = y / 100;
    let g = match (century, gender) {
        (18, Gender::Male) => 1,
        (18, Gender::Female) => 2,
        (19, Gender::Male) => 3,
        (19, Gender::Female) => 4,
        (20, Gender::Male) => 5,
        (20, Gender::Female) => 6,
        _ => 3,
    };
    let serial: u16 = rng.gen_range(1..=999);
    let code_str = format!("{}{:02}{:02}{:02}{:03}", g, y % 100, m, d, serial);
    let digits: Vec<u8> = code_str.bytes().map(|b| b - b'0').collect();
    let check = check_digit(&digits);
    format!("{}{}", code_str, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    digits[10] == check_digit(&digits[..10])
}

pub fn parse(code: &str) -> IdResult {
    let g = code.as_bytes()[0] - b'0';
    let yy: u16 = code[1..3].parse().unwrap_or(0);
    let mm: u8 = code[3..5].parse().unwrap_or(0);
    let dd: u8 = code[5..7].parse().unwrap_or(0);
    let century_base: u16 = match g {
        1 | 2 => 1800,
        3 | 4 => 1900,
        5 | 6 => 2000,
        _ => 1900,
    };
    IdResult {
        code: code.to_string(),
        gender: Some(if g % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century_base + yy, mm, dd)),
        valid: validate(code),
    }
}
