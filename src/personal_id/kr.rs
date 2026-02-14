use rand::Rng;

use super::date::Gender;
use super::IdResult;

static WEIGHTS: [u32; 12] = [2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5];

fn compute_check(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    ((11 - s % 11) % 10) as u8
}

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (year, month, day) = super::date::resolve_date(rng, opts.year);

    let yy = year % 100;
    let century_gender = match (year >= 2000, gender) {
        (false, Gender::Male) => 1u8,
        (false, Gender::Female) => 2,
        (true, Gender::Male) => 3,
        (true, Gender::Female) => 4,
    };

    let region = rng.gen_range(0..=99u8);
    let seq = rng.gen_range(0..=999u16);

    let base = format!(
        "{:02}{:02}{:02}{}{:02}{:03}",
        yy, month, day, century_gender, region,
        seq % 1000
    );
    let digits: Vec<u8> = base.bytes().map(|b| b - b'0').collect();
    let check = compute_check(&digits);
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    compute_check(&digits[..12]) == digits[12]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let (gender, dob) = if clean.len() == 13 {
        let cg = clean.as_bytes()[6] - b'0';
        let g = match cg {
            1 | 3 | 5 | 7 => Some("male".to_string()),
            2 | 4 | 6 | 8 => Some("female".to_string()),
            _ => None,
        };
        let yy: u16 = clean[0..2].parse().unwrap_or(0);
        let century = match cg {
            1 | 2 | 5 | 6 => 1900u16,
            3 | 4 | 7 | 8 => 2000,
            _ => 1900,
        };
        let d = format!(
            "{:04}-{}-{}",
            century + yy,
            &clean[2..4],
            &clean[4..6]
        );
        (g, Some(d))
    } else {
        (None, None)
    };

    IdResult {
        code: if clean.len() == 13 {
            format!("{}-{}", &clean[0..6], &clean[6..13])
        } else {
            clean
        },
        gender,
        dob,
        valid: validate(code),
    }
}
