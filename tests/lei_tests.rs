use idsmith::lei::{GenOptions, Registry};
use rand::thread_rng;

#[test]
fn test_generate_and_validate() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();

    for _ in 0..100 {
        let result = registry.generate(&opts, &mut rng);
        assert_eq!(result.code.len(), 20);
        assert!(result.valid);
        assert!(registry.validate(&result.code));
    }
}

#[test]
fn test_generate_with_country() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions {
        country: Some("DE".to_string()),
    };

    for _ in 0..20 {
        let result = registry.generate(&opts, &mut rng);
        assert_eq!(result.country_code, "DE");
        assert!(registry.validate(&result.code));
    }
}

#[test]
fn test_checksum_corruption_detected() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    let result = registry.generate(&opts, &mut rng);

    // Corrupt one character
    let mut corrupted: Vec<u8> = result.code.bytes().collect();
    corrupted[10] = if corrupted[10] == b'A' { b'B' } else { b'A' };
    let corrupted = String::from_utf8(corrupted).unwrap();

    assert!(!registry.validate(&corrupted));
}

#[test]
fn test_invalid_length() {
    let registry = Registry::new();
    assert!(!registry.validate("ABC"));
    assert!(!registry.validate("123456789012345678901")); // 21 chars
    assert!(!registry.validate("1234567890123456789")); // 19 chars
}

#[test]
fn test_invalid_characters() {
    let registry = Registry::new();
    assert!(!registry.validate("5299 00BOTDR0SE98980")); // space
    assert!(!registry.validate("5299!0BOTDR0SE98980")); // special char
}

#[test]
fn test_lei_fields() {
    let registry = Registry::new();
    let mut rng = thread_rng();
    let opts = GenOptions::default();
    let result = registry.generate(&opts, &mut rng);

    assert_eq!(result.lou, result.code[..4]);
    assert_eq!(result.country_code, result.code[4..6]);
}
