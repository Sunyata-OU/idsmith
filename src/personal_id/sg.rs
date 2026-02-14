use rand::seq::SliceRandom;
use rand::Rng;

use super::IdResult;

static WEIGHTS: [u32; 7] = [2, 7, 6, 5, 4, 3, 2];
static ST_CHECK: &[u8] = b"JZIHGFEDCBA";
static FG_CHECK: &[u8] = b"XWUTRQPNMLK";
static M_CHECK: &[u8] = b"XWUTRQPNMLK";

fn check_letter(prefix: u8, digits: &[u8]) -> char {
    let s: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let offset = match prefix {
        b'T' | b'G' => 4u32,
        b'M' => 3,
        _ => 0,
    };
    let idx = ((s + offset) % 11) as usize;
    let table = match prefix {
        b'S' | b'T' => ST_CHECK,
        b'F' | b'G' => FG_CHECK,
        b'M' => M_CHECK,
        _ => ST_CHECK,
    };
    table[idx] as char
}

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let prefix = *[b'S', b'T'].choose(rng).unwrap();
    let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9u8)).collect();
    let check = check_letter(prefix, &digits);
    let num: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}{}", prefix as char, num, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 9 {
        return false;
    }
    let bytes = code.as_bytes();
    let prefix = bytes[0];
    if !matches!(prefix, b'S' | b'T' | b'F' | b'G' | b'M') {
        return false;
    }
    if !bytes[1..8].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    if !bytes[8].is_ascii_uppercase() {
        return false;
    }
    let digits: Vec<u8> = bytes[1..8].iter().map(|b| b - b'0').collect();
    check_letter(prefix, &digits) == bytes[8] as char
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_uppercase(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
