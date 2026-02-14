use rand::Rng;

use super::{AccountResult, GenOptions};

/// NUBAN (Nigeria Uniform Bank Account Number) check digit.
/// 10-digit account: 3 bank code + 6 serial + 1 check.
/// Weights: 3,7,3,3,7,3,3,7,3 applied to bank code + serial.
fn nuban_check_digit(bank_code: &[u8; 3], serial: &[u8; 6]) -> u8 {
    let weights = [3u8, 7, 3, 3, 7, 3, 3, 7, 3];
    let combined: Vec<u8> = bank_code.iter().chain(serial.iter()).copied().collect();
    let sum: u32 = combined
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    ((10 - sum % 10) % 10) as u8
}

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Bank code: 3 digits (001-999)
    let bank: u16 = rng.gen_range(1..=999);
    let mut bank_digits = [0u8; 3];
    bank_digits[0] = (bank / 100) as u8;
    bank_digits[1] = ((bank / 10) % 10) as u8;
    bank_digits[2] = (bank % 10) as u8;

    // Serial: 6 digits
    let mut serial = [0u8; 6];
    for d in &mut serial {
        *d = rng.gen_range(0..=9);
    }

    let check = nuban_check_digit(&bank_digits, &serial);

    let bank_str = format!("{:03}", bank);
    let account: String = serial
        .iter()
        .chain(std::iter::once(&check))
        .map(|d| (b'0' + d) as char)
        .collect();
    let nuban = format!("{}{}", bank_str, account);

    AccountResult {
        country_code: "NG".into(),
        country_name: crate::countries::get_country_name("NG")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "NUBAN".into(),
        bank_code: Some(bank_str),
        branch_code: None,
        account_number: account,
        check_digits: Some(check.to_string()),
        formatted: nuban.clone(),
        raw: nuban,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() != 10 || !raw.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();
    let mut bank = [0u8; 3];
    let mut serial = [0u8; 6];
    bank.copy_from_slice(&digits[..3]);
    serial.copy_from_slice(&digits[3..9]);
    let check = nuban_check_digit(&bank, &serial);
    digits[9] == check
}

pub fn format(raw: &str) -> String {
    if raw.len() == 10 {
        format!("{} {}", &raw[..3], &raw[3..])
    } else {
        raw.to_string()
    }
}
