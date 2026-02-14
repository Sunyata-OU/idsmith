use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // BSB: 6 digits (XXX-XXX)
    let bsb: String = (0..6)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    // Account number: 5-9 digits
    let acct_len = rng.gen_range(5..=9u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{}-{} {}", &bsb[..3], &bsb[3..], account);
    let raw = format!("{}{}", bsb, account);

    AccountResult {
        country_code: "AU".into(),
        country_name: crate::countries::get_country_name("AU")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "BSB + Account".into(),
        bank_code: Some(bsb),
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
    if raw.len() < 11 || raw.len() > 15 {
        return false;
    }
    raw.chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 6 {
        format!("{}-{} {}", &raw[..3], &raw[3..6], &raw[6..])
    } else {
        raw.to_string()
    }
}
