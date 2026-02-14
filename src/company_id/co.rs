use rand::Rng;

const CHECK_DIGITS: &[u8; 11] = b"01987654321";

/// NIT (Colombian tax number) - 8-15 body digits + 1 check digit.
/// stdnum: weights (3,7,13,17,19,23,29,37,41,43,47,53,59,67,71) applied to reversed body,
/// check = '01987654321'[sum % 11]
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let len = rng.gen_range(8..=9);
    let body: Vec<u8> = (0..len).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [3u32, 7, 13, 17, 19, 23, 29, 37, 41, 43, 47, 53, 59, 67, 71];
    let sum: u32 = body
        .iter()
        .rev()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let check = CHECK_DIGITS[(sum % 11) as usize] - b'0';
    let s: String = body.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}-{}", s, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() < 8 || clean.len() > 16 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let len = digits.len();
    let body = &digits[..len - 1];
    let check = digits[len - 1];
    let weights = [3u32, 7, 13, 17, 19, 23, 29, 37, 41, 43, 47, 53, 59, 67, 71];
    let sum: u32 = body
        .iter()
        .rev()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    let expected = CHECK_DIGITS[(sum % 11) as usize] - b'0';
    expected == check
}
