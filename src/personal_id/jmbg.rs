use rand::Rng;

use super::checksum::weighted_check;
use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const W: &[u8] = &[7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];

fn regions(country: &str) -> &'static [u8] {
    match country {
        "SI" => &[50],
        "RS" => &[
            70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
        ],
        "BA" => &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
        "ME" => &[21, 22, 23, 24, 25, 26, 27, 28, 29],
        _ => &[50],
    }
}

pub fn generate_for(country: &str, opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let region_list = regions(country);
    let rr = region_list[rng.gen_range(0..region_list.len())];

    loop {
        let seq: u16 = if gender == Gender::Male {
            rng.gen_range(0..=499)
        } else {
            rng.gen_range(500..=999)
        };
        let base = format!("{:02}{:02}{:03}{:02}{:03}", d, m, y % 1000, rr, seq);
        let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
        let s = weighted_check(&digits, W, 11);
        let m_val = if s > 0 { 11 - s } else { 0 };
        if m_val == 10 {
            continue;
        }
        let m_val = if m_val == 11 { 0 } else { m_val };
        return format!("{}{}", base, m_val);
    }
}

pub fn generate_si(opts: &GenOptions, rng: &mut impl Rng) -> String {
    generate_for("SI", opts, rng)
}
pub fn generate_rs(opts: &GenOptions, rng: &mut impl Rng) -> String {
    generate_for("RS", opts, rng)
}
pub fn generate_ba(opts: &GenOptions, rng: &mut impl Rng) -> String {
    generate_for("BA", opts, rng)
}
pub fn generate_me(opts: &GenOptions, rng: &mut impl Rng) -> String {
    generate_for("ME", opts, rng)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    if clean.len() != 13 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let s = weighted_check(&digits[..12], W, 11);
    let m_val = if s > 0 { 11 - s } else { 0 };
    let m_val = if m_val >= 10 { 0 } else { m_val };
    m_val == digits[12] as u32
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    let dd: u8 = clean[0..2].parse().unwrap_or(0);
    let mm: u8 = clean[2..4].parse().unwrap_or(0);
    let yyy: u16 = clean[4..7].parse().unwrap_or(0);
    let year: u16 = if yyy >= 900 { 1000 + yyy } else { 2000 + yyy };
    let seq: u16 = clean[9..12].parse().unwrap_or(0);
    IdResult {
        code: clean,
        gender: Some(if seq < 500 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", year, mm, dd)),
        valid: validate(code),
    }
}
