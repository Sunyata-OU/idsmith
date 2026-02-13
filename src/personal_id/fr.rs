use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, _) = date::resolve_date(rng, opts.year);
    let s: u8 = if gender == Gender::Male { 1 } else { 2 };
    // Department 01-95 excluding 20
    let dept: u8 = loop {
        let d = rng.gen_range(1..=95u8);
        if d != 20 {
            break d;
        }
    };
    let commune: u16 = rng.gen_range(1..=999);
    let order: u16 = rng.gen_range(1..=999);
    let num: u64 = format!(
        "{}{:02}{:02}{:02}{:03}{:03}",
        s,
        y % 100,
        m,
        dept,
        commune,
        order
    )
    .parse()
    .unwrap();
    let key = 97 - (num % 97);
    format!(
        "{}{:02}{:02}{:02}{:03}{:03}{:02}",
        s,
        y % 100,
        m,
        dept,
        commune,
        order,
        key
    )
}

pub fn validate(code: &str) -> bool {
    if code.len() != 15 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let num: u64 = code[..13].parse().unwrap_or(0);
    let key: u64 = code[13..15].parse().unwrap_or(0);
    97 - (num % 97) == key
}

pub fn parse(code: &str) -> IdResult {
    let s: u8 = code[0..1].parse().unwrap_or(0);
    let yy: u16 = code[1..3].parse().unwrap_or(0);
    let mm: u8 = code[3..5].parse().unwrap_or(0);
    let century: u16 = if yy > 25 { 1900 } else { 2000 };
    IdResult {
        code: code.to_string(),
        gender: Some(if s == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}", century + yy, mm)),
        valid: validate(code),
    }
}
