use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
        let weights = [7, 9, 10, 5, 8, 4, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let r = sum % 11;
        let check = if r == 0 { 0 } else { 11 - r };
        if check == 10 {
            continue;
        }
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("FI{}{}", s, check);
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("FI") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 8 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [7, 9, 10, 5, 8, 4, 2];
    let sum: u32 = digits[..7]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let r = sum % 11;
    let check = if r == 0 { 0 } else { 11 - r };
    if check == 10 {
        return false;
    }
    check == digits[7] as u32
}
