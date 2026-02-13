use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let century = y / 100;
    // DK CPR century encoding rules:
    //   seq 0001-3999 → always 1900s
    //   seq 4000-4999 → 1937-2036 (yy>=37→1900, yy<=36→2000)
    //   seq 5000-8999 → 1858-1899 or 2000-2057
    //   seq 9000-9999 → 1937-2036
    // Use safe ranges to avoid future-date ambiguity:
    let (seq_min, seq_max) = if century == 19 {
        (1u16, 3999u16)
    } else {
        (4000u16, 9999u16)
    };
    let seq: u16 = loop {
        let s = rng.gen_range(seq_min..=seq_max);
        if (gender == Gender::Male && s % 2 == 1) || (gender == Gender::Female && s % 2 == 0) {
            break s;
        }
    };
    format!("{:02}{:02}{:02}{:04}", d, m, y % 100, seq)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    clean.len() == 10 && clean.chars().all(|c| c.is_ascii_digit())
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    let dd: u8 = clean[0..2].parse().unwrap_or(0);
    let mm: u8 = clean[2..4].parse().unwrap_or(0);
    let yy: u16 = clean[4..6].parse().unwrap_or(0);
    let seq: u16 = clean[6..10].parse().unwrap_or(0);
    let century: u16 = match seq {
        0..=3999 => 1900,
        4000..=4999 => {
            if yy <= 36 {
                2000
            } else {
                1900
            }
        }
        5000..=8999 => {
            if yy <= 57 {
                2000
            } else {
                1800
            }
        }
        _ => {
            if yy <= 36 {
                2000
            } else {
                1900
            }
        }
    };
    IdResult {
        code: format!("{}-{}", &clean[..6], &clean[6..]),
        gender: Some(if seq % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
