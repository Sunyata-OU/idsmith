use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    // CZ 10-digit format (post-1954): yy 54-99→1954-1999, yy 00-24→2000-2024
    // Avoid yy 25-53 which would map to future dates (2025-2053)
    let (y, m, d) = match opts.year {
        Some(yr) => date::rand_date_with_year(rng, yr),
        None => date::rand_date(rng, 1954, 2024),
    };
    let em: u8 = if gender == Gender::Female { m + 50 } else { m };
    let base6 = format!("{:02}{:02}{:02}", y % 100, em, d);
    loop {
        let ext: u16 = rng.gen_range(0..=9999);
        let full: u64 = format!("{}{:04}", base6, ext).parse().unwrap();
        if full.is_multiple_of(11) {
            return format!("{}{:04}", base6, ext);
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '/').collect();
    if clean.len() != 10 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let num: u64 = clean.parse().unwrap_or(1);
    num.is_multiple_of(11)
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '/').collect();
    let yy: u16 = clean[0..2].parse().unwrap_or(0);
    let mm: u8 = clean[2..4].parse().unwrap_or(0);
    let dd: u8 = clean[4..6].parse().unwrap_or(0);
    let gender_str = if mm > 50 { "female" } else { "male" };
    let actual_mm = if mm > 50 { mm - 50 } else { mm };
    let century: u16 = if yy <= 25 { 2000 } else { 1900 };
    IdResult {
        code: format!("{}/{}", &clean[..6], &clean[6..]),
        gender: Some(gender_str.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, actual_mm, dd)),
        valid: validate(code),
    }
}
