use rand::Rng;

use super::checksum::weighted_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W1: &[u8] = &[3, 7, 6, 1, 8, 9, 4, 5, 2];
const W2: &[u8] = &[5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let base = format!("{:02}{:02}{:02}", d, m, y % 100);

    loop {
        let ind: u16 = if y < 2000 {
            rng.gen_range(0..=499)
        } else {
            rng.gen_range(500..=999)
        };
        if (gender == Gender::Male && ind % 2 == 0) || (gender == Gender::Female && ind % 2 == 1) {
            continue;
        }
        let mut digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        digits.push((ind / 100) as u8);
        digits.push(((ind / 10) % 10) as u8);
        digits.push((ind % 10) as u8);

        let r1 = 11u32.wrapping_sub(weighted_check(&digits[..9], W1, 11));
        let r1 = if r1 == 11 { 0 } else { r1 };
        if r1 == 10 {
            continue;
        }
        digits.push(r1 as u8);

        let r2 = 11u32.wrapping_sub(weighted_check(&digits[..10], W2, 11));
        let r2 = if r2 == 11 { 0 } else { r2 };
        if r2 == 10 {
            continue;
        }
        return format!("{}{:03}{}{}", base, ind, r1, r2);
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let d: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let r1 = 11u32.wrapping_sub(weighted_check(&d[..9], W1, 11));
    let r1 = if r1 == 11 { 0 } else { r1 };
    let r2 = 11u32.wrapping_sub(weighted_check(&d[..10], W2, 11));
    let r2 = if r2 == 11 { 0 } else { r2 };
    r1 == d[9] as u32 && r2 == d[10] as u32 && r1 != 10 && r2 != 10
}

pub fn parse(code: &str) -> IdResult {
    let dd: u8 = code[0..2].parse().unwrap_or(0);
    let mm: u8 = code[2..4].parse().unwrap_or(0);
    let yy: u16 = code[4..6].parse().unwrap_or(0);
    let ind: u16 = code[6..9].parse().unwrap_or(0);
    let century: u16 = if ind >= 500 { 2000 } else { 1900 };
    IdResult {
        code: code.to_string(),
        gender: Some(if ind % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
