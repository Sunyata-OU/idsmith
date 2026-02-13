use rand::Rng;

use super::IdResult;

const INVALID_PREFIXES: &[&str] = &["BG", "GB", "KN", "NK", "NT", "TN", "ZZ"];
const FIRST_INVALID: &[u8] = b"DFIQUV";
const SECOND_INVALID: &[u8] = b"DFIOQUV";

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let (c1, c2) = loop {
        let c1 = loop {
            let c = rng.gen_range(b'A'..=b'Z');
            if !FIRST_INVALID.contains(&c) {
                break c;
            }
        };
        let c2 = loop {
            let c = rng.gen_range(b'A'..=b'Z');
            if !SECOND_INVALID.contains(&c) {
                break c;
            }
        };
        let prefix = format!("{}{}", c1 as char, c2 as char);
        if !INVALID_PREFIXES.contains(&prefix.as_str()) {
            break (c1, c2);
        }
    };
    let mut digits = String::new();
    for _ in 0..6 {
        digits.push((b'0' + rng.gen_range(0..=9u8)) as char);
    }
    let suffix = b"ABCD"[rng.gen_range(0..4usize)] as char;
    format!("{}{}{}{}", c1 as char, c2 as char, digits, suffix)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 9 {
        return false;
    }
    let bytes = code.as_bytes();
    bytes[0].is_ascii_uppercase()
        && bytes[1].is_ascii_uppercase()
        && code[2..8].chars().all(|c| c.is_ascii_digit())
        && b"ABCD".contains(&bytes[8])
        && !INVALID_PREFIXES.contains(&&code[..2])
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
