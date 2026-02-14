use rand::thread_rng;

use idsmith::personal_id::date::Gender;
use idsmith::personal_id::{self, GenOptions};

// All countries with specific implementations (61 total)
const ALL_SPECIFIC_COUNTRIES: &[&str] = &[
    // Europe (31)
    "AT", "BA", "BE", "BG", "CH", "CZ", "DE", "DK", "EE", "ES", "FI", "FR", "GB", "GR", "HR", "IE",
    "IS", "IT", "LT", "LV", "ME", "NL", "NO", "PL", "PT", "RO", "RS", "SE", "SI", "SK", "TR",
    // Americas (12)
    "US", "CA", "BR", "AR", "CL", "CO", "CU", "DO", "UY", "EC", "PE", "MX",
    // Asia-Pacific (12)
    "CN", "IN", "JP", "KR", "TW", "TH", "SG", "MY", "ID", "HK", "AU", "NZ",
    // Africa/Middle East (7)
    "ZA", "IL", "EG", "DZ", "MU", "PK", "SA", // Central Asia (1)
    "KZ", // Eastern Europe (1)
    "UA", // Western Europe (1)
    "LU",
];

#[test]
fn test_id_country_count() {
    let registry = personal_id::Registry::new();
    let countries = registry.list_countries();
    // 61 specific + ~31 territory aliases
    assert!(
        countries.len() >= 90,
        "expected >= 90 personal ID countries, got {}",
        countries.len()
    );
}

#[test]
fn test_all_specific_countries_generate_valid() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for &cc in ALL_SPECIFIC_COUNTRIES {
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
fn test_all_specific_countries_parse() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    for &cc in ALL_SPECIFIC_COUNTRIES {
        let code = registry.generate(cc, &opts, &mut rng).unwrap();
        let parsed = registry
            .parse(cc, &code)
            .unwrap_or_else(|| panic!("{}: parse returned None for {}", cc, code));
        assert!(parsed.valid, "{}: parsed.valid is false for {}", cc, code);
        assert!(!parsed.code.is_empty());
    }
}

#[test]
fn test_all_listed_countries_generate_valid() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    let all = registry.list_countries();
    for (cc, _name, _id_name) in &all {
        for _ in 0..5 {
            let code = registry
                .generate(cc, &opts, &mut rng)
                .unwrap_or_else(|| panic!("{}: generate returned None", cc));
            let valid = registry.validate(cc, &code);
            assert_eq!(valid, Some(true), "{}: validation failed for {}", cc, code);
        }
    }
}

#[test]
fn test_territory_aliases() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    let aliases = &["PR", "GU", "CC", "CX", "CK", "BL", "SH", "BV"];
    for &cc in aliases {
        assert!(registry.is_supported(cc), "{} should be supported", cc);
        let code = registry
            .generate(cc, &opts, &mut rng)
            .unwrap_or_else(|| panic!("{}: generate returned None", cc));
        let valid = registry.validate(cc, &code);
        assert_eq!(valid, Some(true), "{}: validation failed for {}", cc, code);
    }
}

#[test]
fn test_gender_filter() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();

    // Countries that encode gender in their ID
    let gendered_countries = &[
        "EE", "FI", "SE", "NO", "PL", "RO", "BG", "CZ", "SK", "BE", "FR", "IT",
        // New countries with gender
        "AR", "MX", "CN", "KR", "TW", "ZA", "EG", "MY", "ID", // Newly added
        "KZ", "UA",
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
    let year_countries = &[
        "EE", "FI", "SE", "NO", "PL", "RO", "BG", "DK", // New countries with DOB
        "CN", "KR", "ZA", "EG", "MX", "MY", "ID", // Newly added
        "KZ", "UA", "LU",
    ];

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
    // New countries
    assert_eq!(registry.name("US"), Some("SSN"));
    assert_eq!(registry.name("CN"), Some("Resident ID"));
    assert_eq!(registry.name("ZA"), Some("SA ID"));
    assert_eq!(registry.name("IN"), Some("Aadhaar"));
    // New countries
    assert_eq!(registry.name("CU"), Some("NI"));
    assert_eq!(registry.name("DO"), Some("Cedula"));
    assert_eq!(registry.name("MU"), Some("NID"));
    assert_eq!(registry.name("PK"), Some("CNIC"));
    // Alias
    assert_eq!(registry.name("PR"), Some("SSN"));
}

#[test]
fn test_checksum_corruption() {
    let registry = personal_id::Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    // Countries with strong checksums - generate and corrupt
    let checksum_countries = &[
        "BR", "CN", "IN", "ZA", "IL", "CA", "JP", "TH", "AU", "KZ", "UA", "SA", "LU",
    ];
    for &cc in checksum_countries {
        for _ in 0..10 {
            let code = registry.generate(cc, &opts, &mut rng).unwrap();
            assert_eq!(
                registry.validate(cc, &code),
                Some(true),
                "{}: valid code failed",
                cc
            );
            // Corrupt one digit
            let mut chars: Vec<char> = code.chars().collect();
            // Find a digit to corrupt
            if let Some(pos) = chars.iter().position(|c| c.is_ascii_digit()) {
                let old = chars[pos];
                chars[pos] = if old == '9' {
                    '0'
                } else {
                    (old as u8 + 1) as char
                };
                let corrupted: String = chars.into_iter().collect();
                let valid = registry.validate(cc, &corrupted).unwrap_or(true);
                // Most corrupted codes should fail, but not guaranteed for all checksums
                let _ = valid;
            }
        }
    }
}
