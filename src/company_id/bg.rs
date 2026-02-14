use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    // 9 digit legal entity
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let mut sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| (i as u32 + 1) * d as u32)
        .sum();
    let mut check = sum % 11;
    if check == 10 {
        sum = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| (i as u32 + 3) * d as u32)
            .sum();
        check = sum % 11;
    }
    digits.push((check % 10) as u8);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code
        .to_uppercase()
        .replace("BG", "")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    if clean.len() == 9 {
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let mut sum: u32 = digits[..8]
            .iter()
            .enumerate()
            .map(|(i, &d)| (i as u32 + 1) * d as u32)
            .sum();
        let mut check = sum % 11;
        if check == 10 {
            sum = digits[..8]
                .iter()
                .enumerate()
                .map(|(i, &d)| (i as u32 + 3) * d as u32)
                .sum();
            check = sum % 11;
        }
        (check % 10) as u8 == digits[8]
    } else if clean.len() == 10 {
        // Person, foreigner, etc.
        // For simplicity, we skip full EGN/PNF but allow standard other
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        let weights = [4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..9]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let expected = (11 - (sum % 11)) % 11;
        expected as u8 == digits[9]
    } else {
        false
    }
}
