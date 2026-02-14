use rand::Rng;

use super::{AccountResult, GenOptions};

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Institution number: 3 digits (001-999)
    let inst: u16 = rng.gen_range(1..=999);
    let inst_str = format!("{:03}", inst);

    // Transit number: 5 digits (00001-99999)
    let transit: u32 = rng.gen_range(1..=99999);
    let transit_str = format!("{:05}", transit);

    // Account number: 7-12 digits
    let acct_len = rng.gen_range(7..=12u8);
    let account: String = (0..acct_len)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect();

    let formatted = format!("{}-{}-{}", inst_str, transit_str, account);
    let raw = format!("{}{}{}", inst_str, transit_str, account);

    AccountResult {
        country_code: "CA".into(),
        country_name: crate::countries::get_country_name("CA")
            .unwrap_or("Unknown")
            .to_string(),
        format_name: "Inst + Transit + Account".into(),
        bank_code: Some(inst_str),
        branch_code: Some(transit_str),
        account_number: account,
        check_digits: None,
        formatted,
        raw,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() < 15 || raw.len() > 20 {
        return false;
    }
    raw.chars().all(|c| c.is_ascii_digit())
}

pub fn format(raw: &str) -> String {
    if raw.len() >= 8 {
        format!("{}-{}-{}", &raw[..3], &raw[3..8], &raw[8..])
    } else {
        raw.to_string()
    }
}
