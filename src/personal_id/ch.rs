use rand::Rng;

use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut base = String::from("756");
    for _ in 0..9 {
        base.push((b'0' + rng.gen_range(0..=9u8)) as char);
    }
    let weights = [1u8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let s: u32 = base
        .bytes()
        .zip(weights.iter())
        .map(|(b, &w)| (b - b'0') as u32 * w as u32)
        .sum();
    let check = (10 - s % 10) % 10;
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '.').collect();
    if clean.len() != 13 || !clean.chars().all(|c| c.is_ascii_digit()) || !clean.starts_with("756")
    {
        return false;
    }
    let weights = [1u8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    let s: u32 = clean[..12]
        .bytes()
        .zip(weights.iter())
        .map(|(b, &w)| (b - b'0') as u32 * w as u32)
        .sum();
    (10 - s % 10) % 10 == (clean.as_bytes()[12] - b'0') as u32
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '.').collect();
    IdResult {
        code: format!(
            "{}.{}.{}.{}",
            &clean[..3],
            &clean[3..7],
            &clean[7..11],
            &clean[11..]
        ),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
