use rand::Rng;

/// CUI/CIF (Romanian company identifier) - 2 to 10 digits, first != 0.
/// stdnum: weights (7,5,3,2,1,7,5,3,2), number zero-padded to 9 digits,
/// check = (10 * sum) % 11 % 10.
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let len = rng.gen_range(2..=9);
    let mut body: Vec<u8> = (0..len).map(|_| rng.gen_range(0..=9)).collect();
    if body[0] == 0 {
        body[0] = rng.gen_range(1..=9);
    }
    // Zero-pad to 9 digits for weight alignment
    let mut padded = vec![0u8; 9 - body.len()];
    padded.extend_from_slice(&body);
    let weights = [7u32, 5, 3, 2, 1, 7, 5, 3, 2];
    let sum: u32 = padded
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let check = ((10 * sum) % 11 % 10) as u8;
    body.push(check);
    body.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("RO") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() < 2 || clean.len() > 10 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    if digits[0] == 0 {
        return false;
    }
    let len = digits.len();
    let body = &digits[..len - 1];
    // Zero-pad body to 9 digits
    let mut padded = vec![0u8; 9 - body.len()];
    padded.extend_from_slice(body);
    let weights = [7u32, 5, 3, 2, 1, 7, 5, 3, 2];
    let sum: u32 = padded
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let expected = ((10 * sum) % 11 % 10) as u8;
    expected == digits[len - 1]
}
