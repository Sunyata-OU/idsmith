use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        // Companies usually start with 5
        let mut digits: Vec<u8> = vec![5];
        for _ in 0..7 {
            digits.push(rng.gen_range(0..=9));
        }
        let weights = [9, 8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let r = sum % 11;
        let check = if r < 2 { 0 } else { 11 - r };
        if check > 9 {
            continue;
        }
        digits.push(check as u8);
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("PT{}", s);
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("PT") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [9, 8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let r = sum % 11;
    let expected = if r < 2 { 0 } else { 11 - r };
    expected == digits[8] as u32
}
