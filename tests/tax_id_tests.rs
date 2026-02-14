use idsmith::tax_id::{GenOptions, Registry};
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
                holder_type: None,
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
fn test_india_pan_format() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("IN".to_string()),
        holder_type: None,
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.code.len(), 10);
    assert!(result.holder_type.is_some());
}

#[test]
fn test_india_pan_with_holder_type() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let opts = GenOptions {
        country: Some("IN".to_string()),
        holder_type: Some("P".to_string()),
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.holder_type, Some("P".to_string()));
    assert_eq!(result.code.chars().nth(3), Some('P'));
}

#[test]
fn test_india_pan_validation() {
    let registry = Registry::new();

    assert!(registry.validate("IN", "ABCPA1234Z"));
    assert!(!registry.validate("IN", "ABCPA123Z")); // too short
    assert!(!registry.validate("IN", "12CPA1234Z")); // digits where letters expected
    assert!(!registry.validate("IN", "ABCXA1234Z")); // bad holder type
}

#[test]
fn test_us_tin_validation() {
    let registry = Registry::new();

    assert!(!registry.validate("US", "000123456")); // area 000
    assert!(!registry.validate("US", "666123456")); // area 666
    assert!(!registry.validate("US", "900123456")); // area 9xx
    assert!(!registry.validate("US", "123004567")); // group 00
    assert!(!registry.validate("US", "123450000")); // serial 0000
}

#[test]
fn test_de_steuer_idnr_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("DE".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 11);
        assert!(
            registry.validate("DE", &result.code),
            "DE Steuer-IdNr validation failed for {}",
            result.code
        );
    }
    // Corrupted should fail
    assert!(!registry.validate("DE", "12345678900"));
}

#[test]
fn test_gb_utr_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("GB".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 10);
        assert!(
            registry.validate("GB", &result.code),
            "GB UTR validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_fr_nif_mod97() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("FR".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 13);
        assert!(
            registry.validate("FR", &result.code),
            "FR NIF validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_cn_usci_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("CN".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 18);
        assert!(
            registry.validate("CN", &result.code),
            "CN USCI validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_br_cpf_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("BR".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 11);
        assert!(
            registry.validate("BR", &result.code),
            "BR CPF validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_au_tfn_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("AU".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("AU", &result.code),
            "AU TFN validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_ca_sin_luhn() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("CA".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("CA", &result.code),
            "CA SIN validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_jp_my_number_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("JP".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 12);
        assert!(
            registry.validate("JP", &result.code),
            "JP My Number validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_it_partita_iva_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("IT".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 11);
        assert!(
            registry.validate("IT", &result.code),
            "IT Partita IVA validation failed for {}",
            result.code
        );
    }
    assert!(!registry.validate("IT", "00000000000")); // all zeros
}

#[test]
fn test_es_nif_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("ES".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("ES", &result.code),
            "ES NIF validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_nl_bsn_11check() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("NL".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("NL", &result.code),
            "NL BSN validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_se_personnummer_luhn() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("SE".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 10);
        assert!(
            registry.validate("SE", &result.code),
            "SE Personnummer validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_kr_brn_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("KR".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 10);
        assert!(
            registry.validate("KR", &result.code),
            "KR BRN validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_sg_nric_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("SG".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 9);
        assert!(
            registry.validate("SG", &result.code),
            "SG Tax Ref validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_za_tax_number_luhn() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("ZA".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.code.len(), 10);
        assert!(
            registry.validate("ZA", &result.code),
            "ZA Tax Number validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_mx_rfc_checksum() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    for _ in 0..50 {
        let opts = GenOptions {
            country: Some("MX".to_string()),
            holder_type: None,
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert!(
            result.code.len() == 12 || result.code.len() == 13,
            "MX RFC unexpected length {}",
            result.code.len()
        );
        assert!(
            registry.validate("MX", &result.code),
            "MX RFC validation failed for {}",
            result.code
        );
    }
}

#[test]
fn test_generic_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let countries = vec!["AT", "CH", "DK", "FI", "NO"];
    for country in countries {
        let opts = GenOptions {
            country: Some(country.to_string()),
            holder_type: None,
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
        holder_type: None,
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
