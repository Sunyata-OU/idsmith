use crate::bank_account::checksum::luhn_check_digit;
use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let province = rng.gen_range(1..=100u16);
    digits.push((province / 100) as u8);
    digits.push(((province / 10) % 10) as u8);
    digits.push((province % 10) as u8);
    let check = luhn_check_digit(&digits);
    digits.push(check);
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("IT{}", s)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("IT") {
        stripped.to_string()
    } else {
        code.to_string()
    };
    if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let province_code: u16 = clean[7..10].parse().unwrap_or(0);
    let valid_province =
        (1..=100).contains(&province_code) || [120, 121, 888, 999].contains(&province_code);
    valid_province && luhn_check_digit(&digits[..10]) == digits[10]
}
