use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let alphabet = "WABCDEFGHIJKLMNOPQRSTUV";
    let check_char = alphabet.chars().nth((sum % 23) as usize).unwrap();
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("IE{}{}", s, check_char)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("IE") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
    };
    if clean.len() < 8 || clean.len() > 9 {
        return false;
    }
    true
}
