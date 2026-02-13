use rand::Rng;

use super::checksum::weighted_check;
use super::date;
use super::{GenOptions, IdResult};

const W: &[u8] = &[3, 2, 7, 6, 5, 4, 3, 2];

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let century_digit: u8 = if y < 2000 { 9 } else { 0 };

    loop {
        let rr: u8 = rng.gen_range(20..=99);
        let base = format!("{:02}{:02}{:02}{:02}", d, m, y % 100, rr);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let r = weighted_check(&digits, W, 11);
        let check = if r == 0 {
            0u8
        } else if r == 1 {
            continue;
        } else {
            (11 - r) as u8
        };
        return format!("{}{}{}", base, check, century_digit);
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    if clean.len() != 10 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean[..8].bytes().map(|b| b - b'0').collect();
    let r = weighted_check(&digits, W, 11);
    let expected = if r == 0 {
        0
    } else if r == 1 {
        return false;
    } else {
        11 - r
    };
    expected == (clean.as_bytes()[8] - b'0') as u32
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    let dd: u8 = clean[0..2].parse().unwrap_or(0);
    let mm: u8 = clean[2..4].parse().unwrap_or(0);
    let yy: u16 = clean[4..6].parse().unwrap_or(0);
    let cent_digit: u8 = clean[9..10].parse().unwrap_or(0);
    let century: u16 = match cent_digit {
        9 => 1900,
        0 => 2000,
        8 => 1800,
        _ => 1900,
    };
    IdResult {
        code: clean.to_string(),
        gender: None,
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
