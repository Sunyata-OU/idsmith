use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = vec![5, 1];
    for _ in 0..6 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = crate::personal_id::checksum::luhn_check(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 || !clean.starts_with('5') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    crate::personal_id::checksum::luhn_check(&digits[..8]) == digits[8]
}
