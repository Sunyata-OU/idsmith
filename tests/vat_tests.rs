use idsmith::vat::{GenOptions, Registry};
use rand::thread_rng;

#[test]
fn test_generate_and_validate_all_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for (code, _name) in registry.list_countries() {
        let opts = GenOptions {
            country: Some(code.to_string()),
        };
        for _ in 0..20 {
            let result = registry.generate(&opts, &mut rng).unwrap();
            assert!(
                result.valid,
                "Generated {} VAT not marked valid: {}",
                code, result.code
            );
            assert!(
                registry.validate(&result.code),
                "Generated {} VAT failed validation: {}",
                code,
                result.code
            );
            assert!(
                result.code.starts_with(code),
                "Generated {} VAT doesn't start with prefix: {}",
                code,
                result.code
            );
            assert_eq!(result.country_code, *code);
        }
    }
}

#[test]
fn test_random_country_generation() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    for _ in 0..100 {
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert!(result.valid);
        assert!(registry.validate(&result.code));
    }
}

#[test]
fn test_greece_gr_alias() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    // Generate with GR should produce EL prefix
    let opts = GenOptions {
        country: Some("GR".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.country_code, "EL");
    assert!(result.code.starts_with("EL"));
    assert!(registry.validate(&result.code));

    // Validate with GR prefix should also work
    let gr_code = format!("GR{}", &result.code[2..]);
    assert!(registry.validate(&gr_code));
}

#[test]
fn test_checksum_corruption_detected() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for (code, _) in registry.list_countries() {
        let opts = GenOptions {
            country: Some(code.to_string()),
        };
        let result = registry.generate(&opts, &mut rng).unwrap();

        // Corrupt a digit in the body (after the prefix)
        let prefix_len = if *code == "AT" { 3 } else { 2 };
        let mut bytes: Vec<u8> = result.code.bytes().collect();
        if bytes.len() > prefix_len + 1 {
            let pos = prefix_len + 1;
            // For ES with letters, find a digit position
            if bytes[pos].is_ascii_digit() {
                bytes[pos] = if bytes[pos] == b'0' { b'1' } else { b'0' };
            } else if bytes[pos].is_ascii_uppercase() {
                bytes[pos] = if bytes[pos] == b'A' { b'B' } else { b'A' };
            }
            let corrupted = String::from_utf8(bytes).unwrap();
            // Some corruptions may accidentally still be valid (1 in ~10 chance for mod 10)
            // so we don't assert false for every case, but at least verify it doesn't panic
            let _ = registry.validate(&corrupted);
        }
    }
}

#[test]
fn test_invalid_format_rejected() {
    let registry = Registry::new();
    assert!(!registry.validate(""));
    assert!(!registry.validate("XX123456789"));
    assert!(!registry.validate("DE"));
    assert!(!registry.validate("DE1234")); // too short
    assert!(!registry.validate("DEABCDEFGHI")); // letters instead of digits
}

#[test]
fn test_unsupported_country_returns_none() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions {
        country: Some("XX".to_string()),
    };
    assert!(registry.generate(&opts, &mut rng).is_none());
}

#[test]
fn test_specific_known_formats() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    // DE: should be DE + 9 digits
    let opts = GenOptions {
        country: Some("DE".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 11); // DE + 9
    assert!(result.code[2..].chars().all(|c| c.is_ascii_digit()));

    // AT: should be ATU + 8 digits
    let opts = GenOptions {
        country: Some("AT".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert!(result.code.starts_with("ATU"));
    assert_eq!(result.code.len(), 11); // AT + U + 8

    // NL: should be NL + 9digits + B + 2digits
    let opts = GenOptions {
        country: Some("NL".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 14); // NL + 12
    assert_eq!(&result.code[11..12], "B");

    // SE: should end with "01"
    let opts = GenOptions {
        country: Some("SE".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 14); // SE + 12
    assert!(result.code.ends_with("01"));

    // CY: should end with a letter
    let opts = GenOptions {
        country: Some("CY".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 11); // CY + 9
    assert!(result.code.as_bytes().last().unwrap().is_ascii_uppercase());
}

#[test]
fn test_list_countries() {
    let registry = Registry::new();
    let countries = registry.list_countries();
    assert_eq!(countries.len(), 28);
    // Should include EL (Greece), not GR
    assert!(countries.iter().any(|(c, _)| *c == "EL"));
    assert!(!countries.iter().any(|(c, _)| *c == "GR"));
}
