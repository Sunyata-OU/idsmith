use rand::Rng;

use super::checksum::weighted_mod11;
use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Bank code: 3 digits (001-999)
    let bank: u16 = rng.gen_range(1..=999);
    let bank_str = format!("{:03}", bank);

    // Branch: 4 digits + check digit
    let branch_digits: Vec<u8> = (0..4).map(|_| rng.gen_range(0..=9)).collect();
    let branch_weights = [5u8, 4, 3, 2];
    let rem = weighted_mod11(&branch_digits, &branch_weights);
    let branch_check: char = match rem {
        0 | 1 => '0',
        _ => (b'0' + (11 - rem) as u8) as char,
    };
    let branch_str: String = branch_digits.iter().map(|d| (b'0' + d) as char).collect();

    // Account number: 6-10 digits + check digit
    let acct_len = rng.gen_range(6..=10u8);
    let acct_digits: Vec<u8> = (0..acct_len).map(|_| rng.gen_range(0..=9)).collect();
    let acct_weights: Vec<u8> = (2..=(acct_len + 1)).rev().collect();
    let acct_rem = weighted_mod11(&acct_digits, &acct_weights);
    let acct_check: char = match acct_rem {
        0 | 1 => '0',
        _ => (b'0' + (11 - acct_rem) as u8) as char,
    };
    let acct_str: String = acct_digits.iter().map(|d| (b'0' + d) as char).collect();

    let formatted = format!(
        "{} {}-{} {}-{}",
        bank_str, branch_str, branch_check, acct_str, acct_check
    );
    let raw = format!(
        "{}{}{}{}{}",
        bank_str, branch_str, branch_check, acct_str, acct_check
    );

    AccountResult {
        country_code: "BR".into(),
        country_name: crate::countries::get_country_name("BR")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "Bank + Branch + Account".into(),
        bank_code: Some(bank_str),
        branch_code: Some(format!("{}-{}", branch_str, branch_check)),
        account_number: format!("{}-{}", acct_str, acct_check),
        check_digits: Some(format!("{}{}", branch_check, acct_check)),
        formatted,
        raw,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() < 15 || raw.len() > 19 {
        return false;
    }
    if !raw.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();

    // Verify branch check digit at position 7
    let branch_weights = [5u8, 4, 3, 2];
    let rem = weighted_mod11(&digits[3..7], &branch_weights);
    let expected_branch: u8 = match rem {
        0 | 1 => 0,
        _ => (11 - rem) as u8,
    };
    if digits[7] != expected_branch {
        return false;
    }

    // Verify account check digit (last digit)
    let acct_digits = &digits[8..raw.len() - 1];
    let acct_len = acct_digits.len() as u8;
    let acct_weights: Vec<u8> = (2..=(acct_len + 1)).rev().collect();
    let acct_rem = weighted_mod11(acct_digits, &acct_weights);
    let expected_acct: u8 = match acct_rem {
        0 | 1 => 0,
        _ => (11 - acct_rem) as u8,
    };
    digits[raw.len() - 1] == expected_acct
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 8 {
        format!("{} {} {}", &raw[..3], &raw[3..8], &raw[8..])
    } else {
        raw.to_string()
    }
}
