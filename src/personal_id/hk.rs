use rand::Rng;

use super::IdResult;

fn letter_val(c: u8) -> u32 {
    (c - b'A' + 1) as u32
}

fn compute_check(prefix: &[u8], digits: &[u8]) -> char {
    let mut sum: u32 = 0;
    if prefix.len() == 1 {
        sum += 36 * 9; // space in position 1 = 36
        sum += letter_val(prefix[0]) * 8;
    } else {
        sum += letter_val(prefix[0]) * 9;
        sum += letter_val(prefix[1]) * 8;
    }
    let weights = [7u32, 6, 5, 4, 3, 2];
    for (d, w) in digits.iter().zip(weights.iter()) {
        sum += *d as u32 * w;
    }
    let r = sum % 11;
    match r {
        0 => '0',
        1 => 'A',
        v => char::from_digit(11 - v, 10).unwrap(),
    }
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let prefix_letter = (b'A' + rng.gen_range(0..26u8)) as char;
    let digits: Vec<u8> = (0..6).map(|_| rng.gen_range(0..=9u8)).collect();
    let check = compute_check(&[prefix_letter as u8], &digits);
    let num: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}({})", prefix_letter, num, check)
}

pub fn validate(code: &str) -> bool {
    // Strip parentheses
    let clean: String = code.chars().filter(|c| *c != '(' && *c != ')').collect();
    if clean.len() < 8 || clean.len() > 9 {
        return false;
    }
    let bytes = clean.as_bytes();
    // Find where digits start
    let prefix_len = if bytes.len() == 9 && bytes[0].is_ascii_uppercase() && bytes[1].is_ascii_uppercase() {
        2
    } else if bytes[0].is_ascii_uppercase() {
        1
    } else {
        return false;
    };
    let digit_part = &bytes[prefix_len..prefix_len + 6];
    if !digit_part.iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let check_char = *bytes.last().unwrap();
    if !check_char.is_ascii_digit() && check_char != b'A' {
        return false;
    }
    let digits: Vec<u8> = digit_part.iter().map(|b| b - b'0').collect();
    let expected = compute_check(&bytes[..prefix_len], &digits);
    expected == check_char as char
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_uppercase(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
