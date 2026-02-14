use rand::Rng;

use super::IdResult;

fn compute_check(digits: &[u8]) -> u8 {
    let s: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 0 {
                let v = d as u32 * 2;
                if v > 9 { v - 9 } else { v }
            } else {
                d as u32
            }
        })
        .sum();
    ((10 - s % 10) % 10) as u8
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let province = rng.gen_range(1..=24u8);
    let type_digit = rng.gen_range(0..=5u8);
    let mut digits: Vec<u8> = Vec::with_capacity(10);
    digits.push(province / 10);
    digits.push(province % 10);
    digits.push(type_digit);
    for _ in 0..6 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = compute_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let province = digits[0] * 10 + digits[1];
    if province < 1 || province > 24 {
        return false;
    }
    if digits[2] > 5 {
        return false;
    }
    compute_check(&digits[..9]) == digits[9]
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
