use rand::Rng;

use super::date::Gender;
use super::IdResult;

// Province codes (first 2 digits)
static PROVINCE_CODES: &[u8] = &[
    11, 12, 13, 14, 15, 16, 17, 18, 19, 21,
    31, 32, 33, 34, 35, 36, 51, 52, 53, 61,
    62, 63, 64, 65, 71, 72, 73, 74, 75, 76,
    81, 82, 91, 92, 94,
];

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (year, month, day) = super::date::resolve_date(rng, opts.year);

    let prov = PROVINCE_CODES[rng.gen_range(0..PROVINCE_CODES.len())];
    let city = rng.gen_range(1..=99u8);
    let district = rng.gen_range(1..=99u8);

    let dd = match gender {
        Gender::Female => day + 40,
        Gender::Male => day,
    };

    let seq = rng.gen_range(1..=9999u16);

    format!(
        "{:02}{:02}{:02}{:02}{:02}{:02}{:04}",
        prov, city, district, dd, month, year % 100, seq
    )
}

pub fn validate(code: &str) -> bool {
    code.len() == 16 && code.chars().all(|c| c.is_ascii_digit())
}

pub fn parse(code: &str) -> IdResult {
    let (gender, dob) = if code.len() == 16 && code.chars().all(|c| c.is_ascii_digit()) {
        let dd: u8 = code[6..8].parse().unwrap_or(0);
        let g = if dd > 40 {
            Some("female".to_string())
        } else {
            Some("male".to_string())
        };
        let actual_day = if dd > 40 { dd - 40 } else { dd };
        let mm = &code[8..10];
        let yy: u16 = code[10..12].parse().unwrap_or(0);
        let year = if yy > 30 { 1900 + yy } else { 2000 + yy };
        (g, Some(format!("{:04}-{}-{:02}", year, mm, actual_day)))
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
