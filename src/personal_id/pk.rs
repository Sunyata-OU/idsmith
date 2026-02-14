use rand::Rng;

use super::date::Gender;
use super::IdResult;

pub fn generate(opts: &super::GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    // Province digit (1-7)
    let province: u8 = rng.gen_range(1..=7);

    // Locality: 4 more digits
    let locality: String = (0..4)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    // Serial: 7 digits
    let serial: String = (0..7)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    // Gender digit: odd = male, even = female (non-zero)
    let gender_digit: u8 = match opts.gender {
        Some(Gender::Male) => rng.gen_range(0..=4) * 2 + 1, // 1,3,5,7,9
        Some(Gender::Female) => rng.gen_range(1..=4) * 2,   // 2,4,6,8
        None => rng.gen_range(1..=9),
    };

    format!("{}{}{}{}", province, locality, serial, gender_digit)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 {
        return false;
    }

    let first = clean.as_bytes()[0] - b'0';
    let last = clean.as_bytes()[12] - b'0';

    // Province must be 1-7
    if !(1..=7).contains(&first) {
        return false;
    }

    // Gender digit must not be 0
    if last == 0 {
        return false;
    }

    true
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let valid = validate(code);

    let gender = if valid && clean.len() == 13 {
        let last = clean.as_bytes()[12] - b'0';
        Some(if last % 2 == 1 { "M" } else { "F" }.to_string())
    } else {
        None
    };

    IdResult {
        country_code: "".to_string(),
        code: if clean.len() == 13 {
            format!("{}-{}-{}", &clean[..5], &clean[5..12], &clean[12..])
        } else {
            clean
        },
        gender,
        dob: None,
        valid,
    }
}
