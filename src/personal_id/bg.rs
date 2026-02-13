use rand::Rng;

use super::checksum::weighted_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W: &[u8] = &[2, 4, 8, 5, 10, 9, 7, 3, 6];

fn month_add(century: u16) -> u8 {
    match century {
        1800 => 20,
        1900 => 0,
        2000 => 40,
        _ => 0,
    }
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let em = m + month_add((y / 100) * 100);
    loop {
        let seq: u16 = rng.gen_range(0..=999);
        if (gender == Gender::Male && seq % 2 == 0) || (gender == Gender::Female && seq % 2 == 1) {
            let base = format!("{:02}{:02}{:02}{:03}", y % 100, em, d, seq);
            let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
            let r = weighted_check(&digits, W, 11);
            let check = if r == 10 { 0 } else { r };
            return format!("{}{}", base, check);
        }
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 10 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let r = weighted_check(&digits[..9], W, 11);
    let expected = if r == 10 { 0 } else { r };
    expected == digits[9] as u32
}

pub fn parse(code: &str) -> IdResult {
    let yy: u16 = code[0..2].parse().unwrap_or(0);
    let mm: u8 = code[2..4].parse().unwrap_or(0);
    let dd: u8 = code[4..6].parse().unwrap_or(0);
    let seq: u16 = code[6..9].parse().unwrap_or(0);
    let (year, month) = if mm > 40 {
        (2000u16 + yy, mm - 40)
    } else if mm > 20 {
        (1800 + yy, mm - 20)
    } else {
        (1900 + yy, mm)
    };
    IdResult {
        code: code.to_string(),
        gender: Some(if seq % 2 == 0 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", year, month, dd)),
        valid: validate(code),
    }
}
