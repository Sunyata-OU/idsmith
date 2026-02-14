use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [5, 4, 3, 2, 7, 6, 5, 4];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let check = (11 - (sum % 11)) % 11;
    digits.push(check as u8);
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("CHE{}", s)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("CHE") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [5, 4, 3, 2, 7, 6, 5, 4];
    let sum: u32 = digits[..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let expected = (11 - (sum % 11)) % 11;
    expected == digits[8] as u32
}
