use rand::Rng;

use super::IdResult;

static WEIGHTS_PRIMARY: [u32; 8] = [3, 2, 7, 6, 5, 4, 3, 2];

fn try_check(digits: &[u8]) -> Option<u8> {
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS_PRIMARY.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let r = s % 11;
    let check = if r == 0 { 0 } else { 11 - r };
    if check <= 9 { Some(check as u8) } else { None }
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    loop {
        let n = rng.gen_range(10_000_000..=150_000_000u32);
        let s = format!("{:09}", n);
        let digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
        // Try with the 8-digit base (last digit is candidate check)
        let base = &digits[..8];
        if let Some(check) = try_check(base) {
            let mut result: Vec<u8> = base.to_vec();
            result.push(check);
            // Strip leading zeros for display
            let num: String = result.iter().map(|d| (b'0' + d) as char).collect();
            let trimmed = num.trim_start_matches('0');
            if trimmed.len() >= 8 {
                return num;
            }
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() < 8 || clean.len() > 9 {
        return false;
    }
    // Pad to 9 digits
    let padded = format!("{:0>9}", clean);
    let digits: Vec<u8> = padded.bytes().map(|b| b - b'0').collect();
    let base = &digits[..8];
    if let Some(check) = try_check(base) {
        check == digits[8]
    } else {
        false
    }
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    IdResult {
        code: clean,
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
