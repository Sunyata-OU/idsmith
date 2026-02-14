use rand::Rng;

use super::checksum::clabe_check_digit;
use super::{AccountResult, GenOptions};

/// Real Mexican bank codes (from Banxico registry).
static BANK_CODES: &[u16] = &[
    2, 6, 9, 12, 14, 19, 21, 22, 30, 32, 36, 37, 42, 44, 58, 59, 60, 62, 72,
    102, 103, 106, 108, 110, 112, 113, 116, 124, 126, 127, 128, 129, 130, 131,
    132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 143, 145, 147, 148, 150,
    151, 152, 154, 155, 156, 157, 158, 159, 160, 166, 167, 168,
];

/// Representative city codes (plaza codes) covering all Mexican states.
static CITY_CODES: &[u16] = &[
    10, 12, 14, 20, 22, 24, 40, 42, 44, 50, 52, 54, 56, 60, 62, 64, 66, 68,
    70, 72, 74, 76, 78, 80, 90, 92, 94, 96, 100, 102, 104, 106, 110, 120, 130,
    140, 150, 152, 154, 160, 170, 180, 190, 192, 194, 200, 210, 212, 214, 220,
    230, 240, 260, 270, 280, 290, 300, 310, 320, 330, 340, 350, 360, 370, 380,
    390, 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, 500, 510, 520, 530,
    540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650, 660, 670, 680,
    690, 700, 710, 720, 730, 740, 750, 760, 770, 780, 790, 800, 810, 820, 830,
    840, 850, 860, 870, 880, 890, 900, 910, 920, 930, 940, 950, 960,
];

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // CLABE: 3 bank + 3 city + 11 account + 1 check = 18 digits
    let mut digits = [0u8; 17];

    // Bank code (from real bank registry)
    let bank = BANK_CODES[rng.gen_range(0..BANK_CODES.len())];
    digits[0] = (bank / 100) as u8;
    digits[1] = ((bank / 10) % 10) as u8;
    digits[2] = (bank % 10) as u8;

    // City code (from real plaza codes)
    let city = CITY_CODES[rng.gen_range(0..CITY_CODES.len())];
    digits[3] = (city / 100) as u8;
    digits[4] = ((city / 10) % 10) as u8;
    digits[5] = (city % 10) as u8;

    // Account (11 digits)
    for d in &mut digits[6..] {
        *d = rng.gen_range(0..=9);
    }
    let check = clabe_check_digit(&digits);
    let clabe: String = digits
        .iter()
        .chain(std::iter::once(&check))
        .map(|d| (b'0' + d) as char)
        .collect();

    let bank_code = clabe[..3].to_string();
    let account = clabe[6..17].to_string();

    AccountResult {
        country_code: "MX".into(),
        country_name: "Mexico".into(),
        format_name: "CLABE".into(),
        bank_code: Some(bank_code),
        branch_code: Some(clabe[3..6].to_string()),
        account_number: account,
        check_digits: Some(check.to_string()),
        formatted: clabe.clone(),
        raw: clabe,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() != 18 || !raw.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = raw.bytes().map(|b| b - b'0').collect();
    let check = clabe_check_digit(&digits[..17]);
    digits[17] == check
}

pub fn format(raw: &str) -> String {
    raw.to_string()
}
