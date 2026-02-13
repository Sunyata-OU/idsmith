use rand::Rng;

use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    loop {
        let mut d: Vec<u8> = vec![rng.gen_range(1..=9)];
        for _ in 0..7 {
            d.push(rng.gen_range(0..=9));
        }
        let s: u32 = (0..8).map(|i| d[i] as u32 * (9 - i) as u32).sum();
        let d9 = s % 11;
        if d9 <= 9 {
            let code: String = d.iter().map(|x| (b'0' + x) as char).collect();
            return format!("{}{}", code, d9);
        }
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 9 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let d: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let s: u32 = (0..8).map(|i| d[i] as u32 * (9 - i) as u32).sum();
    (s.wrapping_sub(d[8] as u32)) % 11 == 0
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
