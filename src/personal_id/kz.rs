use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W1: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const W2: &[u32] = &[3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 2];

fn check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .zip(W1.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r = sum % 11;
    if r < 10 {
        return r as u8;
    }
    let sum2: u32 = digits
        .iter()
        .zip(W2.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r2 = sum2 % 11;
    if r2 < 10 {
        r2 as u8
    } else {
        0
    }
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    loop {
        let (y, m, d) = date::resolve_date(rng, opts.year);
        let century = y / 100;
        let s = match (century, gender) {
            (19, Gender::Male) => 3,
            (19, Gender::Female) => 4,
            (20, Gender::Male) => 5,
            (20, Gender::Female) => 6,
            (21, Gender::Male) => 7,
            (21, Gender::Female) => 8,
            _ => 3,
        };
        let serial: u16 = rng.gen_range(0..=9999);
        let base = format!("{:02}{:02}{:02}{}{:04}", y % 100, m, d, s, serial);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let check = check_digit(&digits);
        // Retry if dual-cycle still gives 10
        let sum2: u32 = digits
            .iter()
            .zip(W2.iter())
            .map(|(&d, &w)| d as u32 * w)
            .sum();
        if sum2 % 11 >= 10 {
            let sum1: u32 = digits
                .iter()
                .zip(W1.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            if sum1 % 11 >= 10 {
                continue;
            }
        }
        return format!("{}{}", base, check);
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 12 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let s = digits[6];
    if !(1..=8).contains(&s) {
        return false;
    }
    digits[11] == check_digit(&digits[..11])
}

pub fn parse(code: &str) -> IdResult {
    let (gender, dob) = if code.len() == 12 && code.chars().all(|c| c.is_ascii_digit()) {
        let s = code.as_bytes()[6] - b'0';
        let g = match s {
            1 | 3 | 5 | 7 => Some("male".to_string()),
            2 | 4 | 6 | 8 => Some("female".to_string()),
            _ => None,
        };
        let century: u16 = match s {
            1 | 2 => 1800,
            3 | 4 => 1900,
            5 | 6 => 2000,
            7 | 8 => 2100,
            _ => 1900,
        };
        let yy: u16 = code[0..2].parse().unwrap_or(0);
        let d = format!("{:04}-{}-{}", century + yy, &code[2..4], &code[4..6]);
        (g, Some(d))
    } else {
        (None, None)
    };

    IdResult {
        country_code: "".to_string(),
        code: code.to_string(),
        gender,
        dob,
        valid: validate(code),
    }
}
