use rand::Rng;

use super::checksum::luhn_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let serial: u16 = loop {
        let s = rng.gen_range(0..=999u16);
        let s3 = s % 10;
        if (gender == Gender::Male && s3 % 2 == 1) || (gender == Gender::Female && s3 % 2 == 0) {
            break s;
        }
    };
    let base = format!("{:02}{:02}{:02}{:03}", y % 100, m, d, serial);
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let check = luhn_check(&digits);
    let sep = if (2025u16.saturating_sub(y)) < 100 {
        '-'
    } else {
        '+'
    };
    format!("{}{}{:03}{}", &base[..6], sep, serial, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '-' && *c != '+').collect();
    if clean.len() != 10 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    luhn_check(&digits[..9]) == digits[9]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '-' && *c != '+').collect();
    let yy: u16 = clean[0..2].parse().unwrap_or(0);
    let mm: u8 = clean[2..4].parse().unwrap_or(0);
    let dd: u8 = clean[4..6].parse().unwrap_or(0);
    let century: u16 = if code.contains('+') {
        1900
    } else if yy <= 25 {
        2000
    } else {
        1900
    };
    IdResult {
        code: code.to_string(),
        gender: Some(
            if clean.as_bytes()[8] % 2 == 1 {
                "male"
            } else {
                "female"
            }
            .to_string(),
        ),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
