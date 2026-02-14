use idsmith::driver_license::{GenOptions, Registry};
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
                state: None,
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
fn test_india_dl_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("IN".to_string()),
        state: None,
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 15);
    assert!(result.state.is_some());
}

#[test]
fn test_india_dl_with_state() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("IN".to_string()),
        state: Some("MH".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert!(result.code.starts_with("MH"));
    assert_eq!(result.state, Some("MH".to_string()));
}

#[test]
fn test_us_dl_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("US".to_string()),
        state: Some("CA".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 13);
    assert!(result.code.chars().next().unwrap().is_ascii_alphabetic());
    assert_eq!(result.state, Some("CA".to_string()));
}

#[test]
fn test_gb_dl_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("GB".to_string()),
        state: None,
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 16);
}

#[test]
fn test_br_cnh_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("BR".to_string()),
            state: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 11);
        assert!(registry.validate("BR", &result.code));
    }
}

#[test]
fn test_es_permiso_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("ES".to_string()),
            state: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("ES", &result.code),
            "ES validate failed for {}",
            result.code
        );
    }
}

#[test]
fn test_se_korkort_luhn() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("SE".to_string()),
            state: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 10);
        assert!(
            registry.validate("SE", &result.code),
            "SE validate failed for {}",
            result.code
        );
    }
}

#[test]
fn test_sg_dl_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("SG".to_string()),
            state: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("SG", &result.code),
            "SG validate failed for {}",
            result.code
        );
    }
}

#[test]
fn test_za_dl_luhn() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("ZA".to_string()),
            state: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 13);
        assert!(
            registry.validate("ZA", &result.code),
            "ZA validate failed for {}",
            result.code
        );
    }
}

#[test]
fn test_invalid_codes() {
    let registry = Registry::new();

    assert!(!registry.validate("IN", "XX0120190000001")); // bad state
    assert!(!registry.validate("IN", "MH01201900000")); // too short
    assert!(!registry.validate("US", "1234567890123")); // no leading letter
    assert!(!registry.validate("GB", "ABCD")); // too short
    assert!(!registry.validate("IT", "ABC")); // too short
    assert!(!registry.validate("NL", "012345678")); // starts with 0 (too short anyway)
}

#[test]
fn test_generic_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let countries = vec!["AT", "CH", "DK", "FI", "NO"];
    for country in countries {
        let opts = GenOptions {
            country: Some(country.to_string()),
            state: None,
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
        state: None,
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
