use idsmith::credit_card::{GenOptions, Registry};
use rand::thread_rng;

#[test]
fn test_credit_card_generation() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    for _ in 0..100 {
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert!(
            registry.validate(&result.number),
            "Failed to validate generated number: {}",
            result.number
        );
        assert!(result.valid);
        assert!(
            result.cvv.len() == 3 || result.cvv.len() == 4,
            "CVV should be 3 or 4 digits, got: {}",
            result.cvv
        );
        assert!(
            result.cvv.chars().all(|c| c.is_ascii_digit()),
            "CVV should be all digits: {}",
            result.cvv
        );
        assert_eq!(result.expiry.len(), 5, "Expiry should be MM/YY");
        assert_eq!(&result.expiry[2..3], "/", "Expiry should have / separator");
    }
}

#[test]
fn test_specific_brands() {
    let registry = Registry::new();
    let mut rng = thread_rng();

    let brands = vec!["visa", "mastercard", "amex", "discover", "jcb", "diners"];

    for brand in brands {
        let opts = GenOptions {
            brand: Some(brand.to_string()),
            ..Default::default()
        };
        let result = registry.generate(&opts, &mut rng).expect(brand);
        assert_eq!(result.brand.to_lowercase(), brand);
        assert!(
            registry.validate(&result.number),
            "Failed to validate {}: {}",
            brand,
            result.number
        );
    }
}

#[test]
fn test_amex_cvv_length() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions {
        brand: Some("amex".to_string()),
        ..Default::default()
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.cvv.len(), 4, "Amex CVV should be 4 digits");
}

#[test]
fn test_non_amex_cvv_length() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    for brand in ["visa", "mastercard", "discover", "jcb", "diners"] {
        let opts = GenOptions {
            brand: Some(brand.to_string()),
            ..Default::default()
        };
        let result = registry.generate(&opts, &mut rng).unwrap();
        assert_eq!(result.cvv.len(), 3, "{} CVV should be 3 digits", brand);
    }
}

#[test]
fn test_amex_length() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions {
        brand: Some("amex".to_string()),
        ..Default::default()
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.number.len(), 15);
}

#[test]
fn test_diners_length() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions {
        brand: Some("diners".to_string()),
        ..Default::default()
    };
    let result = registry.generate(&opts, &mut rng).unwrap();
    assert_eq!(result.number.len(), 14);
}
