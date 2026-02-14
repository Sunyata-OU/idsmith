use idsmith::company_id::{GenOptions, Registry};
use rand::thread_rng;

#[test]
fn test_company_id_generation() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    for _ in 0..100 {
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert!(result.valid);
        assert!(!result.code.is_empty());
    }
}

#[test]
fn test_specific_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let countries = vec!["GB", "DE", "FR", "IT", "ES"];

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
fn test_generic_and_fallback_countries() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    // Generic country (defined in generic.rs but no specific impl)
    let opts_us = GenOptions {
        country: Some("US".to_string()),
    };
    let res_us = registry.generate(&opts_us, &mut rng).unwrap();
    assert_eq!(res_us.country_code, "US");
    assert_eq!(res_us.name, "EIN");

    // Unknown country (not in any list) — should return None
    let opts_zz = GenOptions {
        country: Some("ZZ".to_string()),
    };
    assert!(registry.generate(&opts_zz, &mut rng).is_none());
}
