use rand::Rng;

use super::checksum::luhn_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let seq: u16 = loop {
        let s = rng.gen_range(0..=9999u16);
        if (gender == Gender::Male && s % 2 == 1) || (gender == Gender::Female && s % 2 == 0) {
            break s;
        }
    };
    let base = format!("{:02}{:02}{:02}{:04}", d, m, y % 100, seq);
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let check = luhn_check(&digits);
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code[..10].bytes().map(|b| b - b'0').collect();
    luhn_check(&digits) == code.as_bytes()[10] - b'0'
}

pub fn parse(code: &str) -> IdResult {
    let dd: u8 = code[0..2].parse().unwrap_or(0);
    let mm: u8 = code[2..4].parse().unwrap_or(0);
    let yy: u16 = code[4..6].parse().unwrap_or(0);
    let seq: u16 = code[6..10].parse().unwrap_or(0);
    let century: u16 = if yy <= 25 { 2000 } else { 1900 };
    IdResult {
        code: code.to_string(),
        gender: Some(if seq % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
