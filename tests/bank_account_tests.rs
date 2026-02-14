use rand::thread_rng;

use idsmith::bank_account::{GenOptions, Registry};

#[test]
fn test_account_country_count() {
    let registry = Registry::new();
    let countries = registry.list_countries();
    // 16 specific + 124 IBAN-based + 21 territory aliases = 159+
    assert!(
        countries.len() >= 155,
        "expected at least 155 bank account countries, got {}",
        countries.len()
    );
}

#[test]
fn test_all_countries_generate_and_validate() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for (code, _, _, _) in registry.list_countries() {
        for _ in 0..5 {
            let result = registry
                .generate(code, &opts, &mut rng)
                .unwrap_or_else(|| panic!("generate failed for {}", code));
            assert!(result.valid, "{} generated invalid account", code);
            assert_eq!(result.country_code, code);
            assert!(!result.raw.is_empty(), "{} raw is empty", code);
            assert!(!result.formatted.is_empty(), "{} formatted is empty", code);

            if let Some(valid) = registry.validate(code, &result.raw) {
                assert!(valid, "{} validation failed for {}", code, result.raw);
            }
        }
    }
}

#[test]
fn test_us_aba_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for _ in 0..50 {
        let result = registry.generate("US", &opts, &mut rng).unwrap();
        let routing = result.bank_code.unwrap();
        assert_eq!(routing.len(), 9, "ABA routing must be 9 digits");
        assert!(
            registry.validate("US", &result.raw).unwrap(),
            "US ABA validation failed for {}",
            result.raw
        );
    }
}

#[test]
fn test_mx_clabe_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for _ in 0..50 {
        let result = registry.generate("MX", &opts, &mut rng).unwrap();
        assert_eq!(result.raw.len(), 18, "CLABE must be 18 digits");
        assert!(
            registry.validate("MX", &result.raw).unwrap(),
            "MX CLABE validation failed for {}",
            result.raw
        );
    }
}

#[test]
fn test_cn_luhn_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for _ in 0..50 {
        let result = registry.generate("CN", &opts, &mut rng).unwrap();
        assert!(
            result.raw.len() >= 16 && result.raw.len() <= 19,
            "CN account must be 16-19 digits, got {}",
            result.raw.len()
        );
        assert!(
            registry.validate("CN", &result.raw).unwrap(),
            "CN Luhn validation failed for {}",
            result.raw
        );
    }
}

#[test]
fn test_ar_cbu_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for _ in 0..50 {
        let result = registry.generate("AR", &opts, &mut rng).unwrap();
        assert_eq!(result.raw.len(), 22, "CBU must be 22 digits");
        assert!(
            registry.validate("AR", &result.raw).unwrap(),
            "AR CBU validation failed for {}",
            result.raw
        );
    }
}

#[test]
fn test_ng_nuban_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for _ in 0..50 {
        let result = registry.generate("NG", &opts, &mut rng).unwrap();
        assert_eq!(result.raw.len(), 10, "NUBAN must be 10 digits");
        assert!(
            registry.validate("NG", &result.raw).unwrap(),
            "NG NUBAN validation failed for {}",
            result.raw
        );
    }
}

#[test]
fn test_iban_countries_have_iban() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    let iban_countries = ["DE", "FR", "IT", "ES", "NL", "AT", "SE", "PL", "GB"];
    for &cc in &iban_countries {
        let result = registry.generate(cc, &opts, &mut rng).unwrap();
        assert!(result.iban.is_some(), "{} should have IBAN populated", cc);
        let iban = result.iban.unwrap();
        assert!(
            idsmith::iban::validate_iban(&iban),
            "{} IBAN should be valid: {}",
            cc,
            iban
        );
    }
}

#[test]
fn test_non_iban_countries_no_iban() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    let non_iban = [
        "US", "CA", "MX", "AU", "IN", "JP", "CN", "ZA", "AR", "NG",
    ];
    for &cc in &non_iban {
        let result = registry.generate(cc, &opts, &mut rng).unwrap();
        assert!(
            result.iban.is_none(),
            "{} should not have IBAN, got {:?}",
            cc,
            result.iban
        );
    }
}

#[test]
fn test_unsupported_country() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    assert!(registry.generate("XX", &opts, &mut rng).is_none());
    assert!(registry.validate("XX", "12345").is_none());
    assert!(registry.format("XX", "12345").is_none());
}

#[test]
fn test_territory_aliases() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    // US territories generate US-style accounts
    for &cc in &["PR", "GU", "AS", "VI", "MP", "UM"] {
        let result = registry.generate(cc, &opts, &mut rng).unwrap();
        assert_eq!(result.country_code, cc);
        let routing = result.bank_code.unwrap();
        assert_eq!(routing.len(), 9, "{} should have 9-digit ABA routing", cc);
    }
    // AU territories generate AU-style accounts
    for &cc in &["CC", "CX", "NF", "HM"] {
        let result = registry.generate(cc, &opts, &mut rng).unwrap();
        assert_eq!(result.country_code, cc);
        let bsb = result.bank_code.unwrap();
        assert_eq!(bsb.len(), 6, "{} should have 6-digit BSB", cc);
    }
    // NZ territories
    for &cc in &["CK", "NU", "PN", "TK"] {
        let result = registry.generate(cc, &opts, &mut rng).unwrap();
        assert_eq!(result.country_code, cc);
    }
    // BL (Saint Barthélemy) uses French IBAN
    let result = registry.generate("BL", &opts, &mut rng).unwrap();
    assert_eq!(result.country_code, "BL");
    assert!(result.iban.is_some(), "BL should have IBAN via FR");
}

#[test]
fn test_in_ifsc_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for _ in 0..20 {
        let result = registry.generate("IN", &opts, &mut rng).unwrap();
        let ifsc = result.bank_code.unwrap();
        assert_eq!(ifsc.len(), 11, "IFSC must be 11 chars");
        assert!(
            ifsc[..4].chars().all(|c| c.is_ascii_alphabetic()),
            "IFSC first 4 must be alpha"
        );
        assert_eq!(ifsc.as_bytes()[4], b'0', "IFSC 5th char must be '0'");
    }
}

#[test]
fn test_br_checksum_validation() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    // Verify generated BR accounts pass validation
    for _ in 0..50 {
        let result = registry.generate("BR", &opts, &mut rng).unwrap();
        assert!(
            result.raw.len() >= 15 && result.raw.len() <= 19,
            "BR raw must be 15-19 digits, got {}",
            result.raw.len()
        );
        assert!(
            registry.validate("BR", &result.raw).unwrap(),
            "BR validation failed for {}",
            result.raw
        );
    }

    // Verify that corrupting the branch check digit causes validation to fail
    let result = registry.generate("BR", &opts, &mut rng).unwrap();
    let mut corrupted = result.raw.clone().into_bytes();
    corrupted[7] = if corrupted[7] == b'0' { b'1' } else { b'0' };
    let corrupted_str = String::from_utf8(corrupted).unwrap();
    assert!(
        !registry.validate("BR", &corrupted_str).unwrap(),
        "BR should reject corrupted branch check: {}",
        corrupted_str
    );
}

#[test]
fn test_iban_bban_char_validation() {
    let registry = Registry::new();

    // BG BBAN: Alpha(4) + Numeric(6) + Alphanumeric(8) = 18 chars
    // Valid BBAN with correct char types
    assert!(
        registry.validate("BG", "BNBG966110ABCD1234").unwrap(),
        "BG valid BBAN should pass"
    );
    // Invalid: first 4 are digits instead of alpha
    assert!(
        !registry.validate("BG", "1234966110ABCD1234").unwrap(),
        "BG BBAN with digits in alpha field should fail"
    );
    // Invalid: lowercase in alpha field
    assert!(
        !registry.validate("BG", "bnbg966110ABCD1234").unwrap(),
        "BG BBAN with lowercase in alpha field should fail"
    );

    // DE BBAN: Numeric(8) + Numeric(10) = 18 chars, all numeric
    assert!(
        registry.validate("DE", "370400440532013000").unwrap(),
        "DE valid BBAN should pass"
    );
    // Invalid: letters in numeric field
    assert!(
        !registry.validate("DE", "37040044ABCDEFGHIJ").unwrap(),
        "DE BBAN with alpha in numeric field should fail"
    );

    // Wrong length should fail
    assert!(
        !registry.validate("DE", "12345").unwrap(),
        "DE short BBAN should fail"
    );
}
