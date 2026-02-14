use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // IFSC: 4 alpha + '0' + 6 alphanumeric
    let alpha: String = (0..4)
        .map(|_| (b'A' + rng.gen_range(0..26u8)) as char)
        .collect();
    let suffix: String = (0..6)
        .map(|_| {
            let charset = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            charset[rng.gen_range(0..charset.len())] as char
        })
        .collect();
    let ifsc = format!("{}0{}", alpha, suffix);

    // Account number: 9-18 digits
    let acct_len = rng.gen_range(9..=18u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{} {}", ifsc, account);
    let raw = format!("{}{}", ifsc, account);

    AccountResult {
        country_code: "IN".into(),
        country_name: crate::countries::get_country_name("IN").unwrap_or("Unknown").to_string(),
        format_name: "IFSC + Account".into(),
        bank_code: Some(ifsc),
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
    if raw.len() < 20 || raw.len() > 29 {
        return false;
    }
    let ifsc = &raw[..11];
    // First 4 must be alpha
    if !ifsc[..4].chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    // 5th char must be '0'
    if ifsc.as_bytes()[4] != b'0' {
        return false;
    }
    // Rest of IFSC must be alphanumeric
    if !ifsc[5..].chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }
    // Account must be digits
    raw[11..].chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 11 {
        format!("{} {}", &raw[..11], &raw[11..])
    } else {
        raw.to_string()
    }
}
