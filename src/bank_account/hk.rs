use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Bank code: 3 digits (001-999)
    let bank: u16 = rng.gen_range(1..=999);
    let bank_str = format!("{:03}", bank);

    // Account number: 9-12 digits
    let acct_len = rng.gen_range(9..=12u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{}-{}", bank_str, account);
    let raw = format!("{}{}", bank_str, account);

    AccountResult {
        country_code: "HK".into(),
        country_name: crate::countries::get_country_name("HK").unwrap_or("Unknown").to_string(),
        format_name: "Bank + Account".into(),
        bank_code: Some(bank_str),
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
    if raw.len() < 12 || raw.len() > 15 {
        return false;
    }
    raw.chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 3 {
        format!("{}-{}", &raw[..3], &raw[3..])
    } else {
        raw.to_string()
    }
}
