use rand::Rng;

use super::IdResult;

const LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let num: u32 = rng.gen_range(0..=99_999_999);
    format!("{:08}{}", num, LETTERS[(num % 23) as usize] as char)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 9 || !code[..8].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let num: u32 = code[..8].parse().unwrap_or(0);
    LETTERS[(num % 23) as usize] == code.as_bytes()[8].to_ascii_uppercase()
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
