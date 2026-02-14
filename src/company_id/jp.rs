use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let digits: Vec<u8> = (0..12).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2];
    let sum: u32 = digits
        .iter()
        .rev()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let check = 9 - (sum % 9); // Result 1..9
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}", check, s)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 {
        return false;
    }
    let check = clean.as_bytes()[0] - b'0';
    let body: Vec<u8> = clean.bytes().skip(1).map(|b| b - b'0').collect();
    let weights = [1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2];
    let sum: u32 = body
        .iter()
        .rev()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let expected = 9 - (sum % 9);
    expected == check as u32
}
