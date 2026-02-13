use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let seq: u16 = loop {
        let s = rng.gen_range(1..=997u16);
        if (gender == Gender::Male && s % 2 == 1) || (gender == Gender::Female && s % 2 == 0) {
            break s;
        }
    };
    let num = format!("{:02}{:02}{:02}{:03}", y % 100, m, d, seq);
    let num_val: u64 = num.parse().unwrap();
    let num_for_check: u64 = if y >= 2000 {
        format!("2{}", num).parse().unwrap()
    } else {
        num_val
    };
    let cc = 97 - (num_for_check % 97);
    format!("{}{:02}", num, cc)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '.' && *c != '-').collect();
    if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let num: u64 = clean[..9].parse().unwrap_or(0);
    let cc: u64 = clean[9..11].parse().unwrap_or(0);
    if 97 - (num % 97) == cc {
        return true;
    }
    let num2: u64 = format!("2{}", &clean[..9]).parse().unwrap_or(0);
    97 - (num2 % 97) == cc
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '.' && *c != '-').collect();
    let yy: u16 = clean[0..2].parse().unwrap_or(0);
    let mm: u8 = clean[2..4].parse().unwrap_or(0);
    let dd: u8 = clean[4..6].parse().unwrap_or(0);
    let seq: u16 = clean[6..9].parse().unwrap_or(0);
    let cc: u64 = clean[9..11].parse().unwrap_or(0);
    let num: u64 = clean[..9].parse().unwrap_or(0);
    let year: u16 = if 97 - (num % 97) == cc {
        1900 + yy
    } else {
        2000 + yy
    };
    IdResult {
        code: format!(
            "{}.{}.{}-{}.{}",
            &clean[0..2],
            &clean[2..4],
            &clean[4..6],
            &clean[6..9],
            &clean[9..11]
        ),
        gender: Some(if seq % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", year, mm, dd)),
        valid: validate(code),
    }
}
