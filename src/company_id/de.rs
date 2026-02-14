use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = loop {
        let d = (0..8).map(|_| rng.gen_range(0..=9)).collect::<Vec<u8>>();
        if d[0] != 0 {
            break d;
        }
    };
    let check = crate::personal_id::checksum::iso7064_mod11_10(&digits);
    digits.push(check);
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("DE{}", s)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("DE") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if clean.starts_with('0') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    crate::personal_id::checksum::iso7064_mod11_10(&digits[..8]) == digits[8]
}
