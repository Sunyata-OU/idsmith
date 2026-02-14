use rand::Rng;

use super::checksum::luhn_check_digit;
use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Chinese bank accounts: 16-19 digits with Luhn check
    let total_len = rng.gen_range(16..=19u8);
    let payload_len = total_len - 1;
    let mut digits: Vec<u8> = (0..payload_len).map(|_| rng.gen_range(0..=9)).collect();
    // First digit should not be 0
    if digits[0] == 0 {
        digits[0] = rng.gen_range(1..=9);
    }
    let check = luhn_check_digit(&digits);
    digits.push(check);

    let raw: String = digits.iter().map(|d| (b'0' + d) as char).collect();

    // Format with spaces every 4 digits
    let formatted = raw
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ");

    AccountResult {
        country_code: "CN".into(),
        country_name: "China".into(),
        format_name: "Bank Account (Luhn)".into(),
        bank_code: None,
        branch_code: None,
        account_number: raw.clone(),
        check_digits: Some(check.to_string()),
        formatted,
        raw,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() < 16 || raw.len() > 19 {
        return false;
    }
    if !raw.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();
    let check = luhn_check_digit(&digits[..digits.len() - 1]);
    *digits.last().unwrap() == check
}

pub fn format(raw: &str) -> String {
    raw.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
