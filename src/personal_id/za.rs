use rand::Rng;

use super::checksum;
use super::date::Gender;
use super::IdResult;

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (year, month, day) = super::date::resolve_date(rng, opts.year);

    let seq = match gender {
        Gender::Female => rng.gen_range(0..=4999u16),
        Gender::Male => rng.gen_range(5000..=9999u16),
    };
    let citizen = 0u8; // SA citizen
    let filler = 8u8;

    let base = format!(
        "{:02}{:02}{:02}{:04}{}{}",
        year % 100, month, day, seq, citizen, filler
    );
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let check = checksum::luhn_check(&digits);
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 13 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    checksum::luhn_check(&digits[..12]) == digits[12]
}

pub fn parse(code: &str) -> IdResult {
    let (gender, dob) = if code.len() == 13 && code.chars().all(|c| c.is_ascii_digit()) {
        let seq: u16 = code[6..10].parse().unwrap_or(0);
        let g = if seq >= 5000 {
            Some("male".to_string())
        } else {
            Some("female".to_string())
        };
        let yy: u16 = code[0..2].parse().unwrap_or(0);
        let year = if yy > 30 { 1900 + yy } else { 2000 + yy };
        let d = format!("{:04}-{}-{}", year, &code[2..4], &code[4..6]);
        (g, Some(d))
    } else {
        (None, None)
    };

    IdResult {
        code: code.to_string(),
        gender,
        dob,
        valid: validate(code),
    }
}
