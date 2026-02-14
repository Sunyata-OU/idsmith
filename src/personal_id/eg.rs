use rand::Rng;

use super::checksum;
use super::date::Gender;
use super::IdResult;

// Governorate codes
static GOVS: &[u8] = &[
    1, 2, 3, 4, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 33, 34, 35, 88,
];

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (year, month, day) = super::date::resolve_date(rng, opts.year);

    let century = if year >= 2000 { 3u8 } else { 2 };
    let gov = GOVS[rng.gen_range(0..GOVS.len())];

    // Sequence: odd = male, even = female, 3 digits + gender digit
    let seq_base = rng.gen_range(0..=499u16);
    let seq = match gender {
        Gender::Male => seq_base * 2 + 1,   // odd
        Gender::Female => seq_base * 2,      // even
    };

    let base = format!(
        "{}{:02}{:02}{:02}{:02}{:04}",
        century,
        year % 100,
        month,
        day,
        gov,
        seq
    );
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let check = checksum::luhn_check(&digits);
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 14 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let century = code.as_bytes()[0] - b'0';
    if century != 2 && century != 3 {
        return false;
    }
    let digits: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    checksum::luhn_check(&digits[..13]) == digits[13]
}

pub fn parse(code: &str) -> IdResult {
    let (gender, dob) = if code.len() == 14 && code.chars().all(|c| c.is_ascii_digit()) {
        let seq: u16 = code[9..13].parse().unwrap_or(0);
        let g = if seq % 2 == 1 {
            Some("male".to_string())
        } else {
            Some("female".to_string())
        };
        let century_d = code.as_bytes()[0] - b'0';
        let century: u16 = if century_d == 3 { 2000 } else { 1900 };
        let yy: u16 = code[1..3].parse().unwrap_or(0);
        let d = format!("{:04}-{}-{}", century + yy, &code[3..5], &code[5..7]);
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
