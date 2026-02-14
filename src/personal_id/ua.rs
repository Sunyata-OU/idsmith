use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const WEIGHTS: &[i32] = &[-1, 5, 7, 9, 4, 6, 10, 5, 7];

fn check_digit(digits: &[u8]) -> u8 {
    let sum: i32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as i32 * w)
        .sum();
    ((sum % 11) % 10) as u8
}

fn is_leap(y: u16) -> bool {
    y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400))
}

fn days_in_year(y: u16) -> u32 {
    if is_leap(y) {
        366
    } else {
        365
    }
}

fn day_of_year(y: u16, m: u8, d: u8) -> u32 {
    let days_before: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let mut doy = days_before[(m - 1) as usize] + d as u32;
    if m > 2 && is_leap(y) {
        doy += 1;
    }
    doy
}

fn days_since_epoch(y: u16, m: u8, d: u8) -> u32 {
    // Days since 1900-01-01
    let mut total: u32 = 0;
    for year in 1900..y {
        total += days_in_year(year);
    }
    total += day_of_year(y, m, d) - 1;
    total
}

fn date_from_days(days: u32) -> (u16, u8, u8) {
    let mut remaining = days;
    let mut y: u16 = 1900;
    loop {
        let dy = days_in_year(y);
        if remaining < dy {
            break;
        }
        remaining -= dy;
        y += 1;
    }
    let days_per_month: [u8; 12] = [
        31,
        if is_leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m: u8 = 1;
    for &dm in &days_per_month {
        if remaining < dm as u32 {
            break;
        }
        remaining -= dm as u32;
        m += 1;
    }
    (y, m, remaining as u8 + 1)
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);

    let days = days_since_epoch(y, m, d);
    let days_str = format!("{:05}", days);

    // Digits 6-9: 4-digit number where last digit encodes gender (odd=male, even=female)
    let serial: u16 = rng.gen_range(0..=499);
    let gender_num = match gender {
        Gender::Male => serial * 2 + 1,
        Gender::Female => serial * 2,
    };

    let base = format!("{}{:04}", days_str, gender_num);
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let check = check_digit(&digits);
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 10 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    digits[9] == check_digit(&digits[..9])
}

pub fn parse(code: &str) -> IdResult {
    let (gender, dob) = if code.len() == 10 && code.chars().all(|c| c.is_ascii_digit()) {
        let pen = code.as_bytes()[8] - b'0';
        let g = if pen % 2 == 1 {
            Some("male".to_string())
        } else {
            Some("female".to_string())
        };

        let days: u32 = code[0..5].parse().unwrap_or(0);
        let (y, m, d) = date_from_days(days);
        let dob_str = Some(format!("{:04}-{:02}-{:02}", y, m, d));
        (g, dob_str)
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
