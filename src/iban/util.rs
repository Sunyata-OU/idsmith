use rand::Rng;

use super::types::CharType;

pub(crate) fn random_chars(rng: &mut impl Rng, length: u8, char_type: CharType) -> String {
    let charset: &[u8] = match char_type {
        CharType::Numeric => b"0123456789",
        CharType::Alpha => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        CharType::Alphanumeric => b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    };
    (0..length)
        .map(|_| charset[rng.gen_range(0..charset.len())] as char)
        .collect()
}

pub(crate) fn random_digits(rng: &mut impl Rng, length: u8) -> String {
    random_chars(rng, length, CharType::Numeric)
}
