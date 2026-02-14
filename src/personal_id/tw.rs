use rand::Rng;

use super::date::Gender;
use super::IdResult;

// Letter -> two-digit numeric value for checksum
fn letter_value(c: u8) -> (u8, u8) {
    let vals: &[(u8, u8, u8)] = &[
        (b'A', 1, 0), (b'B', 1, 1), (b'C', 1, 2), (b'D', 1, 3),
        (b'E', 1, 4), (b'F', 1, 5), (b'G', 1, 6), (b'H', 1, 7),
        (b'I', 3, 4), (b'J', 1, 8), (b'K', 1, 9), (b'L', 2, 0),
        (b'M', 2, 1), (b'N', 2, 2), (b'O', 3, 5), (b'P', 2, 3),
        (b'Q', 2, 4), (b'R', 2, 5), (b'S', 2, 6), (b'T', 2, 7),
        (b'U', 2, 8), (b'V', 2, 9), (b'W', 3, 2), (b'X', 3, 0),
        (b'Y', 3, 1), (b'Z', 3, 3),
    ];
    for &(ch, d1, d2) in vals {
        if c == ch {
            return (d1, d2);
        }
    }
    (0, 0)
}

fn compute_check(letter: u8, digits: &[u8]) -> u8 {
    let (d1, d2) = letter_value(letter);
    let mut sum = d1 as u32 * 1 + d2 as u32 * 9;
    let weights = [8u32, 7, 6, 5, 4, 3, 2, 1];
    for (d, w) in digits.iter().zip(weights.iter()) {
        sum += *d as u32 * w;
    }
    ((10 - sum % 10) % 10) as u8
}

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let letter = (b'A' + rng.gen_range(0..26u8)) as char;
    let gender_digit: u8 = match gender {
        Gender::Male => 1,
        Gender::Female => 2,
    };
    let mut digits: Vec<u8> = Vec::with_capacity(9);
    digits.push(gender_digit);
    for _ in 0..7 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = compute_check(letter as u8, &digits);
    digits.push(check);
    let num: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}", letter, num)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 10 {
        return false;
    }
    let bytes = code.as_bytes();
    if !bytes[0].is_ascii_uppercase() {
        return false;
    }
    if !bytes[1..].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let gender_d = bytes[1] - b'0';
    if gender_d != 1 && gender_d != 2 {
        return false;
    }
    let digits: Vec<u8> = bytes[1..9].iter().map(|b| b - b'0').collect();
    let expected = compute_check(bytes[0], &digits);
    expected == bytes[9] - b'0'
}

pub fn parse(code: &str) -> IdResult {
    let gender = if code.len() == 10 {
        match code.as_bytes()[1] {
            b'1' => Some("male".to_string()),
            b'2' => Some("female".to_string()),
            _ => None,
        }
    } else {
        None
    };
    IdResult {
        code: code.to_string(),
        gender,
        dob: None,
        valid: validate(code),
    }
}
