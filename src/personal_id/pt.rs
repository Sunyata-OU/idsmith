use rand::Rng;

use super::IdResult;

const W: &[u8] = &[9, 8, 7, 6, 5, 4, 3, 2];

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut d: Vec<u8> = vec![rng.gen_range(1..=2)];
    for _ in 0..7 {
        d.push(rng.gen_range(0..=9));
    }
    let s: u32 = d
        .iter()
        .zip(W.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let r = s % 11;
    let check = if r < 2 { 0 } else { 11 - r };
    let code: String = d.iter().map(|x| (b'0' + x) as char).collect();
    format!("{}{}", code, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 9 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let d: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let s: u32 = d[..8]
        .iter()
        .zip(W.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let r = s % 11;
    let expected = if r < 2 { 0 } else { 11 - r };
    expected == d[8] as u32
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
