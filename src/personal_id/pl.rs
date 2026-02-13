use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W: &[u8] = &[1, 3, 7, 9, 1, 3, 7, 9, 1, 3];

fn month_add(century: u16) -> u8 {
    match century {
        1800 => 80,
        1900 => 0,
        2000 => 20,
        2100 => 40,
        2200 => 60,
        _ => 0,
    }
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let em = m + month_add((y / 100) * 100);
    loop {
        let zzz: u16 = rng.gen_range(0..=999);
        let x: u8 = rng.gen_range(0..=9);
        if (gender == Gender::Male && x % 2 == 1)
            || (gender == Gender::Female && x.is_multiple_of(2))
        {
            let base = format!("{:02}{:02}{:02}{:03}{}", y % 100, em, d, zzz, x);
            let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
            let s: u32 = digits
                .iter()
                .zip(W.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum::<u32>()
                % 10;
            let check = (10 - s) % 10;
            return format!("{}{}", base, check);
        }
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let s: u32 = digits[..10]
        .iter()
        .zip(W.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum::<u32>()
        % 10;
    (10 - s) % 10 == digits[10] as u32
}

pub fn parse(code: &str) -> IdResult {
    let yy: u16 = code[0..2].parse().unwrap_or(0);
    let mm: u8 = code[2..4].parse().unwrap_or(0);
    let dd: u8 = code[4..6].parse().unwrap_or(0);
    let (year, month) = if mm > 80 {
        (1800u16 + yy, mm - 80)
    } else if mm > 60 {
        (2200 + yy, mm - 60)
    } else if mm > 40 {
        (2100 + yy, mm - 40)
    } else if mm > 20 {
        (2000 + yy, mm - 20)
    } else {
        (1900 + yy, mm)
    };
    IdResult {
        code: code.to_string(),
        gender: Some(
            if (code.as_bytes()[9] - b'0') % 2 == 1 {
                "male"
            } else {
                "female"
            }
            .to_string(),
        ),
        dob: Some(format!("{}-{:02}-{:02}", year, month, dd)),
        valid: validate(code),
    }
}
