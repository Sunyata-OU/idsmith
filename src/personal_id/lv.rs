use rand::Rng;

use super::IdResult;

const W: &[u8] = &[1, 6, 3, 7, 9, 10, 5, 8, 4, 2];

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut base: Vec<u8> = vec![3, 2];
    for _ in 0..8 {
        base.push(rng.gen_range(0..=9));
    }
    let s: u32 = base
        .iter()
        .zip(W.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let check = (1101u32.wrapping_sub(s)) % 11 % 10;
    let code: String = base.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}", code, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let d: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let s: u32 = d[..10]
        .iter()
        .zip(W.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    (1101u32.wrapping_sub(s)) % 11 % 10 == d[10] as u32
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    IdResult {
        code: clean,
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
