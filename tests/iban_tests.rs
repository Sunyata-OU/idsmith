use rand::thread_rng;

// We need to reference the crate; Cargo uses the package name with hyphens replaced by underscores.
use idsmith::iban;

const ALL_IBAN_COUNTRIES: &[&str] = &[
    "AD", "AE", "AL", "AT", "AX", "AZ", "BA", "BE", "BG", "BH", "BR", "BY", "CH", "CR", "CY", "CZ",
    "DE", "DK", "DO", "EE", "EG", "ES", "FI", "FO", "FR", "GB", "GE", "GF", "GI", "GL", "GP", "GR",
    "GT", "HR", "HU", "IE", "IL", "IQ", "IS", "IT", "JO", "KW", "KZ", "LB", "LC", "LI", "LT", "LU",
    "LV", "LY", "MC", "MD", "ME", "MF", "MK", "MN", "MQ", "MR", "MT", "MU", "NC", "NI", "NL", "NO",
    "OM", "PF", "PK", "PL", "PM", "PS", "PT", "QA", "RE", "RO", "RS", "RU", "SA", "SC", "SD", "SE",
    "SI", "SK", "SM", "SO", "ST", "SV", "TF", "TL", "TN", "TR", "UA", "VA", "VG", "WF", "XK", "YT",
];

#[test]
fn test_iban_country_count() {
    let countries = iban::supported_countries();
    assert_eq!(countries.len(), 96, "expected 96 IBAN countries");
}

#[test]
fn test_all_iban_countries_generate_valid() {
    let mut rng = thread_rng();
    for &cc in ALL_IBAN_COUNTRIES {
        for _ in 0..10 {
            let result = iban::generate_iban(Some(cc), &mut rng);
            assert!(result.is_ok(), "{}: generation failed: {:?}", cc, result);
            let code = result.unwrap();
            assert!(
                iban::validate_iban(&code),
                "{}: mod-97 validation failed for {}",
                cc,
                code
            );
        }
    }
}

#[test]
fn test_iban_format_spacing() {
    let formatted = iban::format_iban("DE89370400440532013000");
    assert_eq!(formatted, "DE89 3704 0044 0532 0130 00");
}

#[test]
fn test_iban_random_country() {
    let mut rng = thread_rng();
    for _ in 0..50 {
        let result = iban::generate_iban(None, &mut rng);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(iban::validate_iban(&code), "random IBAN invalid: {}", code);
    }
}

#[test]
fn test_iban_unsupported_country() {
    let mut rng = thread_rng();
    let result = iban::generate_iban(Some("XX"), &mut rng);
    assert!(result.is_err());
}

#[test]
fn test_iban_case_insensitive() {
    let mut rng = thread_rng();
    let result = iban::generate_iban(Some("de"), &mut rng);
    assert!(result.is_ok());
}

#[test]
fn test_iban_validate_known_good() {
    // Known valid IBANs (from Wikipedia / IBAN examples)
    assert!(iban::validate_iban("GB29 NWBK 6016 1331 9268 19"));
    assert!(iban::validate_iban("DE89370400440532013000"));
    assert!(iban::validate_iban("FR7630006000011234567890189"));
}

#[test]
fn test_iban_validate_known_bad() {
    assert!(!iban::validate_iban("GB29 NWBK 6016 1331 9268 18")); // wrong check
    assert!(!iban::validate_iban("XX00")); // too short
    assert!(!iban::validate_iban("")); // empty
}
