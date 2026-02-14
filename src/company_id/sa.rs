use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut s = String::from("3");
    for _ in 0..13 {
        s.push((b'0' + rng.gen_range(0..=9u8)) as char);
    }
    let digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();
    let check = crate::personal_id::checksum::luhn_check(&digits);
    s.push((b'0' + check) as char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 15 || !clean.starts_with('3') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    crate::personal_id::checksum::luhn_check(&digits[..14]) == digits[14]
}
