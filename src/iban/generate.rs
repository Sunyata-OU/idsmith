use rand::Rng;

use super::checksum::fix_bban_checksums;
use super::countries::{get_format, supported_countries};
use super::util::random_chars;

fn generate_bban(country: &str, rng: &mut impl Rng) -> Option<String> {
    let fields = get_format(country)?;
    let mut bban = String::new();
    for field in fields {
        bban.push_str(&random_chars(rng, field.length, field.char_type));
    }
    fix_bban_checksums(country, &mut bban, rng);
    Some(bban)
}

fn letter_to_digits(ch: char) -> String {
    if ch.is_ascii_uppercase() {
        format!("{}", ch as u32 - 'A' as u32 + 10)
    } else {
        ch.to_string()
    }
}

fn iban_mod97(numeric_str: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in numeric_str.chars() {
        remainder = remainder * 10 + ch.to_digit(10).unwrap() as u64;
        remainder %= 97;
    }
    remainder
}

fn calculate_check_digits(country_code: &str, bban: &str) -> String {
    let mut rearranged = String::new();
    for ch in bban.chars() {
        rearranged.push_str(&letter_to_digits(ch));
    }
    for ch in country_code.chars() {
        rearranged.push_str(&letter_to_digits(ch));
    }
    rearranged.push_str("00");
    let remainder = iban_mod97(&rearranged);
    format!("{:02}", 98 - remainder)
}

/// Generate a random valid IBAN for the given country code.
///
/// If `country` is `None`, a random supported country is chosen.
/// Returns an error if the country code is not supported.
///
/// # Examples
///
/// ```
/// use rand::thread_rng;
/// use eu_test_data_generator::iban;
///
/// let mut rng = thread_rng();
/// let code = iban::generate_iban(Some("DE"), &mut rng).unwrap();
/// assert!(code.starts_with("DE"));
/// assert!(iban::validate_iban(&code));
/// ```
pub fn generate_iban(country: Option<&str>, rng: &mut impl Rng) -> Result<String, String> {
    let cc = match country {
        Some(c) => {
            let c = c.to_uppercase();
            if get_format(&c).is_none() {
                return Err(format!("Unsupported country: {}", c));
            }
            c
        }
        None => {
            let countries = supported_countries();
            countries[rng.gen_range(0..countries.len())].to_string()
        }
    };
    let bban = generate_bban(&cc, rng).unwrap();
    let check = calculate_check_digits(&cc, &bban);
    Ok(format!("{}{}{}", cc, check, bban))
}

/// Format an IBAN with spaces every 4 characters for display.
///
/// # Examples
///
/// ```
/// use eu_test_data_generator::iban;
/// assert_eq!(iban::format_iban("DE89370400440532013000"), "DE89 3704 0044 0532 0130 00");
/// ```
pub fn format_iban(iban: &str) -> String {
    iban.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Validate an IBAN using the ISO 13616 mod-97 check.
///
/// Accepts IBANs with or without spaces.
///
/// # Examples
///
/// ```
/// use eu_test_data_generator::iban;
/// assert!(iban::validate_iban("GB29 NWBK 6016 1331 9268 19"));
/// assert!(!iban::validate_iban("GB29 NWBK 6016 1331 9268 18"));
/// ```
pub fn validate_iban(iban: &str) -> bool {
    let clean: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.len() < 4 {
        return false;
    }
    let rearranged: String = clean[4..].to_string() + &clean[..4];
    let numeric: String = rearranged.chars().map(letter_to_digits).collect();
    iban_mod97(&numeric) == 1
}
