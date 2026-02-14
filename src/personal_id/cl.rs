use rand::Rng;

use super::IdResult;

fn compute_check(body: u32) -> char {
    let s = format!("{:08}", body);
    let digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
    let weights = [3u32, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r = 11 - (sum % 11);
    match r {
        11 => '0',
        10 => 'K',
        v => char::from_digit(v, 10).unwrap(),
    }
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let body = rng.gen_range(1_000_000..=99_999_999u32);
    let check = compute_check(body);
    format!("{}{}", body, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    if clean.len() < 8 || clean.len() > 9 {
        return false;
    }
    let body_str = &clean[..clean.len() - 1];
    let check_char = clean.chars().last().unwrap().to_ascii_uppercase();
    if !body_str.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let body: u32 = body_str.parse().unwrap_or(0);
    if body == 0 {
        return false;
    }
    compute_check(body) == check_char
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
