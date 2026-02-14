use rand::Rng;

use super::checksum;
use super::date;
use super::{GenOptions, IdResult};

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let serial: u16 = rng.gen_range(0..=999);
    let base = format!("{:04}{:02}{:02}{:03}", y, m, d, serial);
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let c1 = checksum::luhn_check(&digits);
    let c2 = checksum::verhoeff_check(&digits);
    format!("{}{}{}", base, c1, c2)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 13 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let c1 = checksum::luhn_check(&digits[..11]);
    let c2 = checksum::verhoeff_check(&digits[..11]);
    digits[11] == c1 && digits[12] == c2
}

pub fn parse(code: &str) -> IdResult {
    let dob = if code.len() == 13 && code.chars().all(|c| c.is_ascii_digit()) {
        Some(format!("{}-{}-{}", &code[0..4], &code[4..6], &code[6..8]))
    } else {
        None
    };

    IdResult {
        country_code: "".to_string(),
        code: code.to_string(),
        gender: None,
        dob,
        valid: validate(code),
    }
}
