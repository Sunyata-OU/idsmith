use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let mut checksum = 0;
    for &d in &digits {
        checksum = checksum * 2 + d as u32;
    }
    let check = (checksum * 2 % 11) % 10;
    digits.push(check as u8);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("EL") {
        stripped.to_string()
    } else if let Some(stripped) = code.strip_prefix("GR") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    let clean = if clean.len() == 8 {
        format!("0{}", clean)
    } else {
        clean
    };
    if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let mut checksum = 0;
    for &d in &digits[..8] {
        checksum = checksum * 2 + d as u32;
    }
    let expected = (checksum * 2 % 11) % 10;
    expected == digits[8] as u32
}
