use rand::Rng;

/// REGON (Polish register of economic units) - 9 digits.
/// stdnum: weights (8,9,2,3,4,5,6,7), check = sum % 11 % 10.
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [8u32, 9, 2, 3, 4, 5, 6, 7];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let check = (sum % 11) % 10;
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}", s, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [8u32, 9, 2, 3, 4, 5, 6, 7];
    let sum: u32 = digits[..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    (sum % 11 % 10) == digits[8] as u32
}
