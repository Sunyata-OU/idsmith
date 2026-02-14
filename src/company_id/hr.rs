use crate::personal_id::checksum::iso7064_mod11_10;
use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..10).map(|_| rng.gen_range(0..=9)).collect();
    let check = iso7064_mod11_10(&digits);
    digits.push(check);
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("HR{}", s)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("HR") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    iso7064_mod11_10(&digits[..10]) == digits[10]
}
