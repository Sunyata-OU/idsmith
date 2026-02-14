use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Branch code: 6 digits
    let branch: String = (0..6)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    // Account number: 7-11 digits
    let acct_len = rng.gen_range(7..=11u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{} {}", branch, account);
    let raw = format!("{}{}", branch, account);

    AccountResult {
        country_code: "ZA".into(),
        country_name: crate::countries::get_country_name("ZA").unwrap_or("Unknown").to_string(),
        format_name: "Branch + Account".into(),
        bank_code: Some(branch),
        branch_code: None,
        account_number: account,
        check_digits: None,
        formatted,
        raw,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() < 13 || raw.len() > 17 {
        return false;
    }
    raw.chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 6 {
        format!("{} {}", &raw[..6], &raw[6..])
    } else {
        raw.to_string()
    }
}
