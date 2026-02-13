use rand::Rng;

use super::IdResult;

const MAP: &[u8] = b"WABCDEFGHIJKLMNOPQRSTUV";

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9u8)).collect();
    let s: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| d as u32 * (8 - i) as u32)
        .sum();
    let check = MAP[(s % 23) as usize] as char;
    let code: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}W", code, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() < 8 || !code[..7].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let mut s: u32 = (0..7)
        .map(|i| (code.as_bytes()[i] - b'0') as u32 * (8 - i) as u32)
        .sum();
    if code.len() == 9 && code.as_bytes()[8].is_ascii_alphabetic() {
        let idx = MAP
            .iter()
            .position(|&c| c == code.as_bytes()[8])
            .unwrap_or(0);
        s += idx as u32 * 9;
    }
    MAP[(s % 23) as usize] == code.as_bytes()[7]
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
