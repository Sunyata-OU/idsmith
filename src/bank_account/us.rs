use rand::Rng;

use super::checksum::aba_check_digit;
use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // ABA routing number: first 2 digits are Federal Reserve district (01-12)
    let fed_prefix: u8 = rng.gen_range(1..=12);
    let mut digits = [0u8; 8];
    digits[0] = fed_prefix / 10;
    digits[1] = fed_prefix % 10;
    for d in &mut digits[2..] {
        *d = rng.gen_range(0..=9);
    }
    let check = aba_check_digit(&digits);
    let routing: String = digits
        .iter()
        .chain(std::iter::once(&check))
        .map(|d| (b'0' + d) as char)
        .collect();

    let acct_len = rng.gen_range(8..=17u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{} {}", routing, account);
    let raw = format!("{}{}", routing, account);

    AccountResult {
        country_code: "US".into(),
        country_name: crate::countries::get_country_name("US").unwrap_or("Unknown").to_string(),
        format_name: "ABA Routing + Account".into(),
        bank_code: Some(routing),
        branch_code: None,
        account_number: account,
        check_digits: Some(check.to_string()),
        formatted,
        raw,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() < 17 {
        return false;
    }
    let routing = &raw[..9];
    if !routing.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = routing.bytes().map(|b| b - b'0').collect();
    let check = aba_check_digit(&digits[..8]);
    digits[8] == check
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 9 {
        format!("{} {}", &raw[..9], &raw[9..])
    } else {
        raw.to_string()
    }
}
