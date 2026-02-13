use rand::thread_rng;

use eu_test_data_generator::personal_id::date::Gender;
use eu_test_data_generator::personal_id::{self, GenOptions};

const ALL_ID_COUNTRIES: &[&str] = &[
    "AT", "BA", "BE", "BG", "CH", "CZ", "DE", "DK", "EE", "ES", "FI", "FR", "GB", "GR", "HR", "IE",
    "IS", "IT", "LT", "LV", "ME", "NL", "NO", "PL", "PT", "RO", "RS", "SE", "SI", "SK", "TR",
];

#[test]
fn test_id_country_count() {
    let registry = personal_id::Registry::new();
    let countries = registry.list_countries();
    assert_eq!(countries.len(), 31, "expected 31 personal ID countries");
}

#[test]
fn test_all_id_countries_generate_valid() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for &cc in ALL_ID_COUNTRIES {
        for _ in 0..20 {
            let code = registry
                .generate(cc, &opts, &mut rng)
                .unwrap_or_else(|| panic!("{}: generate returned None", cc));
            let valid = registry.validate(cc, &code);
            assert_eq!(valid, Some(true), "{}: validation failed for {}", cc, code);
        }
    }
}

#[test]
fn test_all_id_countries_parse() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for &cc in ALL_ID_COUNTRIES {
        let code = registry.generate(cc, &opts, &mut rng).unwrap();
        let parsed = registry
            .parse(cc, &code)
            .unwrap_or_else(|| panic!("{}: parse returned None for {}", cc, code));
        assert!(parsed.valid, "{}: parsed.valid is false for {}", cc, code);
        assert!(!parsed.code.is_empty());
    }
}

#[test]
fn test_gender_filter() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();

    // Countries that encode gender in their ID
    let gendered_countries = &[
        "EE", "FI", "SE", "NO", "PL", "RO", "BG", "CZ", "SK", "BE", "FR", "IT",
    ];

    for &cc in gendered_countries {
        let opts_m = GenOptions {
            gender: Some(Gender::Male),
            year: None,
        };
        let opts_f = GenOptions {
            gender: Some(Gender::Female),
            year: None,
        };

        for _ in 0..5 {
            let code_m = registry.generate(cc, &opts_m, &mut rng).unwrap();
            let parsed_m = registry.parse(cc, &code_m).unwrap();
            if let Some(ref g) = parsed_m.gender {
                assert_eq!(g, "male", "{}: expected male, got {} for {}", cc, g, code_m);
            }

            let code_f = registry.generate(cc, &opts_f, &mut rng).unwrap();
            let parsed_f = registry.parse(cc, &code_f).unwrap();
            if let Some(ref g) = parsed_f.gender {
                assert_eq!(
                    g, "female",
                    "{}: expected female, got {} for {}",
                    cc, g, code_f
                );
            }
        }
    }
}

#[test]
fn test_year_filter() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();

    // Countries that encode birth year in their ID
    let year_countries = &["EE", "FI", "SE", "NO", "PL", "RO", "BG", "DK"];

    for &cc in year_countries {
        let opts = GenOptions {
            gender: None,
            year: Some(1985),
        };
        for _ in 0..5 {
            let code = registry.generate(cc, &opts, &mut rng).unwrap();
            let parsed = registry.parse(cc, &code).unwrap();
            if let Some(ref dob) = parsed.dob {
                assert!(
                    dob.starts_with("1985-"),
                    "{}: expected year 1985 in dob, got {} for {}",
                    cc,
                    dob,
                    code
                );
            }
        }
    }
}

#[test]
fn test_unsupported_country() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    assert!(registry.generate("XX", &opts, &mut rng).is_none());
    assert!(registry.validate("XX", "12345").is_none());
    assert!(registry.parse("XX", "12345").is_none());
    assert!(registry.name("XX").is_none());
}

#[test]
fn test_registry_name() {
    let registry = personal_id::Registry::new();
    assert_eq!(registry.name("EE"), Some("Isikukood"));
    assert_eq!(registry.name("IT"), Some("Codice Fiscale"));
    assert_eq!(registry.name("PL"), Some("PESEL"));
    assert_eq!(registry.name("GB"), Some("NINO"));
}
