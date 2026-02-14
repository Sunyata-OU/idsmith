use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Bank code: 4 digits
    let bank: u16 = rng.gen_range(1..=9999);
    let bank_str = format!("{:04}", bank);

    // Branch code: 3 digits
    let branch: u16 = rng.gen_range(1..=999);
    let branch_str = format!("{:03}", branch);

    // Account number: 6-10 digits
    let acct_len = rng.gen_range(6..=10u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{}-{}-{}", bank_str, branch_str, account);
    let raw = format!("{}{}{}", bank_str, branch_str, account);

    AccountResult {
        country_code: "SG".into(),
        country_name: crate::countries::get_country_name("SG")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "Bank + Branch + Account".into(),
        bank_code: Some(bank_str),
        branch_code: Some(branch_str),
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
    if raw.len() >= 7 {
        format!("{}-{}-{}", &raw[..4], &raw[4..7], &raw[7..])
    } else {
        raw.to_string()
    }
}
