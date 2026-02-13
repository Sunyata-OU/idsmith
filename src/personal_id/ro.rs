use rand::Rng;

use super::checksum::weighted_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W: &[u8] = &[2, 7, 9, 1, 4, 6, 3, 5, 8, 2, 7, 9];

fn sex_digit(century: u16, gender: Gender) -> u8 {
    match (century, gender) {
        (1900, Gender::Male) => 1,
        (1900, Gender::Female) => 2,
        (1800, Gender::Male) => 3,
        (1800, Gender::Female) => 4,
        (2000, Gender::Male) => 5,
        (2000, Gender::Female) => 6,
        _ => 5,
    }
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let century = (y / 100) * 100;
    let s = sex_digit(century, gender);
    let county: u8 = rng.gen_range(1..=46);
    let seq: u16 = rng.gen_range(1..=999);
    let base = format!("{}{:02}{:02}{:02}{:02}{:03}", s, y % 100, m, d, county, seq);
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let r = weighted_check(&digits, W, 11);
    let check = if r == 10 { 1 } else { r };
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 13 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let r = weighted_check(&digits[..12], W, 11);
    let expected = if r == 10 { 1 } else { r };
    expected == digits[12] as u32
}

pub fn parse(code: &str) -> IdResult {
    let s: u8 = code[0..1].parse().unwrap_or(0);
    let yy: u16 = code[1..3].parse().unwrap_or(0);
    let mm: u8 = code[3..5].parse().unwrap_or(0);
    let dd: u8 = code[5..7].parse().unwrap_or(0);
    let century: u16 = match s {
        1 | 2 => 1900,
        3 | 4 => 1800,
        5 | 6 => 2000,
        _ => 1900,
    };
    IdResult {
        code: code.to_string(),
        gender: Some(if s % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
