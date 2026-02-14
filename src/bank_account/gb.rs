use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Sort code: 6 digits (XX-XX-XX)
    let sort_code: String = (0..6)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    // Account number: 8 digits
    let account: String = (0..8)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!(
        "{}-{}-{} {}",
        &sort_code[..2],
        &sort_code[2..4],
        &sort_code[4..],
        account
    );
    let raw = format!("{}{}", sort_code, account);

    AccountResult {
        country_code: "GB".into(),
        country_name: crate::countries::get_country_name("GB").unwrap_or("Unknown").to_string(),
        format_name: "Sort Code + Account".into(),
        bank_code: Some(sort_code),
        branch_code: None,
        account_number: account,
        check_digits: None,
        formatted,
        raw,
        iban: None, // filled by Registry if has_iban
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() != 14 {
        return false;
    }
    raw.chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    if raw.len() == 14 {
        format!("{}-{}-{} {}", &raw[..2], &raw[2..4], &raw[4..6], &raw[6..])
    } else {
        raw.to_string()
    }
}
