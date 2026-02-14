use rand::Rng;

use super::IdResult;

static WEIGHTS: [u32; 9] = [1, 4, 3, 7, 5, 8, 6, 9, 10];

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    loop {
        let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9u8)).collect();
        // Find check digit that makes weighted sum divisible by 11
        for check in 0..=9u8 {
            let mut all = digits.clone();
            all.push(check);
            let s: u32 = all
                .iter()
                .zip(WEIGHTS.iter())
                .map(|(&d, &w)| d as u32 * w)
                .sum();
            if s % 11 == 0 {
                return all.iter().map(|d| (b'0' + d) as char).collect();
            }
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    s % 11 == 0
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: if clean.len() == 9 {
            format!("{} {} {}", &clean[0..3], &clean[3..6], &clean[6..9])
        } else {
            clean
        },
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
