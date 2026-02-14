use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // South Korean bank accounts: 11-14 digits
    let total_len = rng.gen_range(11..=14u8);
    let account: String = (0..total_len)
        .map(|i| {
            if i == 0 {
                (b'0' + rng.gen_range(1..=9u8)) as char
            } else {
                (b'0' + rng.gen_range(0..=9u8)) as char
            }
        })
        .collect();

    // Format as groups: XXX-XX-XXXXXX or XXX-XXXX-XXXX-XX
    let formatted = match account.len() {
        11 => format!("{}-{}-{}", &account[..3], &account[3..5], &account[5..]),
        12 => format!("{}-{}-{}", &account[..3], &account[3..6], &account[6..]),
        13 => format!(
            "{}-{}-{}-{}",
            &account[..3],
            &account[3..7],
            &account[7..11],
            &account[11..]
        ),
        14 => format!(
            "{}-{}-{}-{}",
            &account[..3],
            &account[3..7],
            &account[7..11],
            &account[11..]
        ),
        _ => account.clone(),
    };

    AccountResult {
        country_code: "KR".into(),
        country_name: crate::countries::get_country_name("KR")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "Bank Account".into(),
        bank_code: None,
        branch_code: None,
        account_number: account.clone(),
        check_digits: None,
        formatted,
        raw: account,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() < 11 || raw.len() > 14 {
        return false;
    }
    raw.chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    match raw.len() {
        11 => format!("{}-{}-{}", &raw[..3], &raw[3..5], &raw[5..]),
        12 => format!("{}-{}-{}", &raw[..3], &raw[3..6], &raw[6..]),
        13 | 14 => format!(
            "{}-{}-{}-{}",
            &raw[..3],
            &raw[3..7],
            &raw[7..11],
            &raw[11..]
        ),
        _ => raw.to_string(),
    }
}
