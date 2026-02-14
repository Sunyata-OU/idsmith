use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        if digits[0] == 0 {
            continue;
        }
        let weights = [3, 4, 6, 7, 8, 9, 10, 1];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        if sum % 37 == 0 {
            let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            return format!("MT{}", s);
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("MT") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 8 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [3, 4, 6, 7, 8, 9, 10, 1];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    sum % 37 == 0
}
