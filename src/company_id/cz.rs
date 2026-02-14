use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    // Standard legal entity (8 digits)
    loop {
        let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
        if digits[0] == 9 {
            continue;
        }
        let weights = [8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = (11 - (sum % 11)) % 11;
        let check_digit = (if check == 0 { 1 } else { check }) % 10;
        digits.push(check_digit as u8);
        return digits.iter().map(|d| (b'0' + d) as char).collect();
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("CZ") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() == 8 {
        if clean.starts_with('9') {
            return false;
        }
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..7]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = (11 - (sum % 11)) % 11;
        let expected = (if check == 0 { 1 } else { check }) % 10;
        expected == digits[7] as u32
    } else if clean.len() == 9 && clean.starts_with('6') {
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[1..8]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = sum % 11;
        let expected = (8 - (10 - check) % 11) % 10;
        expected == digits[8] as u32
    } else if clean.len() == 9 || clean.len() == 10 {
        crate::personal_id::cz::validate(&clean)
    } else {
        false
    }
}
