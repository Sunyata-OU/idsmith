use idsmith::passport::{GenOptions, Registry};
use rand::thread_rng;

const SPECIFIC_COUNTRIES: &[&str] = &[
    "AE", "AR", "AT", "AU", "BD", "BE", "BG", "BH", "BR", "CA", "CH", "CL", "CN", "CO", "CZ", "DE",
    "DK", "DZ", "EC", "EE", "EG", "ES", "ET", "FI", "FR", "GB", "GH", "GR", "HK", "HR", "HU", "ID",
    "IE", "IL", "IN", "IS", "IT", "JP", "KE", "KR", "KW", "LK", "LT", "LU", "LV", "MA", "MT", "MX",
    "MY", "NG", "NL", "NO", "NP", "NZ", "OM", "PE", "PH", "PK", "PL", "PT", "QA", "RO", "RS", "SA",
    "SE", "SG", "SI", "SK", "TH", "TN", "TR", "TW", "TZ", "UA", "US", "UY", "VE", "VN", "ZA",
];

#[test]
fn test_all_specific_countries_generate_valid() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for &cc in SPECIFIC_COUNTRIES {
        for _ in 0..20 {
            let opts = GenOptions {
                country: Some(cc.to_string()),
            };
            let result = registry
                .generate(&opts, &mut rng)
                .unwrap_or_else(|| panic!("{}: generate failed", cc));
            assert_eq!(result.country_code, cc);
            assert!(result.valid, "{}: not valid", cc);
            assert!(
                registry.validate(cc, &result.code),
                "{}: validate failed for {}",
                cc,
                result.code
            );
        }
    }
}

#[test]
fn test_india_passport_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("IN".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 8);
    let chars: Vec<char> = result.code.chars().collect();
    assert!(chars[0].is_ascii_uppercase());
    assert!(chars[1..].iter().all(|c| c.is_ascii_digit()));
}

#[test]
fn test_us_passport_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("US".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 9);
    assert!(result.code.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn test_de_passport_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("DE".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 9);
    assert!(result.code.starts_with('C'));
}

#[test]
fn test_cn_passport_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("CN".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 9);
    let first = result.code.chars().next().unwrap();
    assert!(first == 'E' || first == 'G');
}

#[test]
fn test_kr_passport_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("KR".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 9);
    let first = result.code.chars().next().unwrap();
    assert!(first == 'M' || first == 'S');
}

#[test]
fn test_sg_passport_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("SG".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 9);
    assert!(result.code.starts_with('E'));
    let chars: Vec<char> = result.code.chars().collect();
    assert!(chars[8].is_ascii_alphabetic());
}

#[test]
fn test_passport_validation() {
    let registry = Registry::new();

    // Valid
    assert!(registry.validate("IN", "A1234567"));
    assert!(registry.validate("US", "123456789"));
    assert!(registry.validate("DE", "C12AB34CD"));
    assert!(registry.validate("CN", "E12345678"));
    assert!(registry.validate("CN", "G12345678"));
    assert!(registry.validate("IT", "AB1234567"));
    assert!(registry.validate("ES", "ABC123456"));
    assert!(registry.validate("KR", "M12345678"));
    assert!(registry.validate("SG", "E1234567A"));

    // Invalid
    assert!(!registry.validate("IN", "12345678")); // no leading letter
    assert!(!registry.validate("IN", "A123456")); // too short
    assert!(!registry.validate("US", "12345")); // too short
    assert!(!registry.validate("DE", "A12345678")); // wrong prefix
    assert!(!registry.validate("CN", "A12345678")); // wrong prefix (not E/G)
    assert!(!registry.validate("KR", "A12345678")); // wrong prefix (not M/S)
    assert!(!registry.validate("SG", "A1234567B")); // wrong prefix (not E)
}

#[test]
fn test_generic_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let countries = vec!["AT", "CH", "DK", "FI", "NO"];
    for country in countries {
        let opts = GenOptions {
            country: Some(country.to_string()),
        };
        let result = registry.generate(&opts, &mut rng).expect(country);
        assert_eq!(result.country_code, country);
        assert!(result.valid);
    }
}

#[test]
fn test_unknown_country_returns_none() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("ZZ".to_string()),
    };
    assert!(registry.generate(&opts, &mut rng).is_none());
}

#[test]
fn test_list_countries() {
    let registry = Registry::new();
    let countries = registry.list_countries();
    assert!(countries.len() >= 75);
    let codes: Vec<&str> = countries.iter().map(|(c, _, _)| *c).collect();
    for &cc in SPECIFIC_COUNTRIES {
        assert!(codes.contains(&cc), "missing {}", cc);
    }
}

#[test]
fn test_random_country_generation() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    for _ in 0..20 {
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert!(result.valid);
        assert!(!result.code.is_empty());
    }
}
