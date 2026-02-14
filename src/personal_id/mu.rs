use rand::Rng;

use super::IdResult;

const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn alpha_index(c: u8) -> u32 {
    match c {
        b'0'..=b'9' => (c - b'0') as u32,
        b'A'..=b'Z' => (c - b'A' + 10) as u32,
        _ => 0,
    }
}

fn calc_check_digit(number: &[u8]) -> u8 {
    let sum: u32 = number
        .iter()
        .enumerate()
        .map(|(i, &c)| (14 - i as u32) * alpha_index(c))
        .sum();
    ALPHABET[((17u32.wrapping_sub(sum % 17)) % 17) as usize]
}

pub fn generate(_opts: &super::GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    // First char: A-Z (first letter of surname)
    let first = (b'A' + rng.gen_range(0..26u8)) as char;

    // Day (01-28), month (01-12), year (00-99)
    let day: u8 = rng.gen_range(1..=28);
    let month: u8 = rng.gen_range(1..=12);
    let year: u8 = rng.gen_range(0..=99);

    // 6-digit serial
    let serial: String = (0..6)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let partial = format!("{}{:02}{:02}{:02}{}", first, day, month, year, serial);
    let check = calc_check_digit(partial.as_bytes());

    format!("{}{}", partial, check as char)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_uppercase();

    if clean.len() != 14 {
        return false;
    }

    let bytes = clean.as_bytes();

    // First char must be A-Z
    if !bytes[0].is_ascii_uppercase() {
        return false;
    }

    // Chars 1-12 must be digits
    if !bytes[1..13].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }

    // Last char is check digit (digit or letter)
    if !bytes[13].is_ascii_alphanumeric() {
        return false;
    }

    // Validate date
    let day = clean[1..3].parse::<u8>().unwrap_or(0);
    let month = clean[3..5].parse::<u8>().unwrap_or(0);
    if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
        return false;
    }

    // Check digit
    calc_check_digit(&bytes[..13]) == bytes[13]
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_uppercase();

    let valid = validate(code);
    let dob = if valid && clean.len() == 14 {
        let day = clean[1..3].parse::<u8>().unwrap_or(0);
        let month = clean[3..5].parse::<u8>().unwrap_or(0);
        let year = clean[5..7].parse::<u16>().unwrap_or(0);
        // Century is ambiguous, assume 1900s or 2000s
        let full_year = if year > 25 { 1900 + year } else { 2000 + year };
        Some(format!("{:04}-{:02}-{:02}", full_year, month, day))
    } else {
        None
    };

    IdResult {
        country_code: "".to_string(),
        code: clean,
        gender: None,
        dob,
        valid,
    }
}
