use rand::Rng;

use super::date::Gender;
use super::IdResult;

// Place of birth codes (selected common ones)
static PB_CODES: &[u8] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
    74, 75, 76, 77, 78, 79, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 98, 99,
];

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (year, month, day) = super::date::resolve_date(rng, opts.year);
    let pb = PB_CODES[rng.gen_range(0..PB_CODES.len())];
    let seq = rng.gen_range(0..=999u16);
    let last = match gender {
        Gender::Male => seq * 2 + 1,   // odd
        Gender::Female => seq * 2,     // even
    };
    let last_digit = (last % 10) as u8;
    let seq_part = last / 10;
    format!(
        "{:02}{:02}{:02}{:02}{:03}{}",
        year % 100, month, day, pb, seq_part, last_digit
    )
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 12 {
        return false;
    }
    clean.chars().all(|c| c.is_ascii_digit())
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let (gender, dob) = if clean.len() == 12 {
        let last = clean.as_bytes()[11] - b'0';
        let g = if last % 2 == 1 {
            Some("male".to_string())
        } else {
            Some("female".to_string())
        };
        let yy: u16 = clean[0..2].parse().unwrap_or(0);
        let mm = &clean[2..4];
        let dd = &clean[4..6];
        // Assume 1900s if yy > 30, else 2000s
        let year = if yy > 30 { 1900 + yy } else { 2000 + yy };
        (g, Some(format!("{:04}-{}-{}", year, mm, dd)))
    } else {
        (None, None)
    };

    IdResult {
        code: if clean.len() == 12 {
            format!("{}-{}-{}", &clean[0..6], &clean[6..8], &clean[8..12])
        } else {
            clean
        },
        gender,
        dob,
        valid: validate(code),
    }
}
