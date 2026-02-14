use rand::Rng;

use super::{AccountResult, GenOptions};

/// CBU (Clave Bancaria Uniforme) mod-10 check digit.
/// Weights (3,1,7,9) applied from right to left (reversed).
fn cbu_check_digit(digits: &[u8]) -> u8 {
    let weights = [3u8, 1, 7, 9];
    let sum: u32 = digits
        .iter()
        .rev()
        .zip(weights.iter().cycle())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    ((10 - sum % 10) % 10) as u8
}

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // CBU: 22 digits = block1 (8) + block2 (14)
    // Block 1: bank (3) + branch (4) + check1 (1)
    let bank: u16 = rng.gen_range(1..=999);
    let branch: u16 = rng.gen_range(1..=9999);
    let b1_digits: Vec<u8> = format!("{:03}{:04}", bank, branch)
        .bytes()
        .map(|b| b - b'0')
        .collect();
    let check1 = cbu_check_digit(&b1_digits);

    // Block 2: account (13) + check2 (1)
    let acct_digits: Vec<u8> = (0..13).map(|_| rng.gen_range(0..=9u8)).collect();
    let check2 = cbu_check_digit(&acct_digits);

    let block1: String = b1_digits
        .iter()
        .chain(std::iter::once(&check1))
        .map(|d| (b'0' + d) as char)
        .collect();
    let block2: String = acct_digits
        .iter()
        .chain(std::iter::once(&check2))
        .map(|d| (b'0' + d) as char)
        .collect();

    let raw = format!("{}{}", block1, block2);
    let formatted = format!("{} {}", block1, block2);

    AccountResult {
        country_code: "AR".into(),
        country_name: crate::countries::get_country_name("AR")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "CBU".into(),
        bank_code: Some(format!("{:03}", bank)),
        branch_code: Some(format!("{:04}", branch)),
        account_number: acct_digits.iter().map(|d| (b'0' + d) as char).collect(),
        check_digits: Some(format!("{}{}", check1, check2)),
        formatted,
        raw,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() != 22 || !raw.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();
    let check1 = cbu_check_digit(&digits[..7]);
    if digits[7] != check1 {
        return false;
    }
    let check2 = cbu_check_digit(&digits[8..21]);
    digits[21] == check2
}

pub fn format(raw: &str) -> String {
    if raw.len() == 22 {
        format!("{} {}", &raw[..8], &raw[8..])
    } else {
        raw.to_string()
    }
}
