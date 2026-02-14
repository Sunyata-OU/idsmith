use rand::Rng;

/// ABN (Australian Business Number) - 11 digits, first two are check digits.
/// stdnum: calc_check_digits: weights (3,5,7,9,11,13,15,17,19) on body digits,
/// check = str(11 + (sum_of(-w*d) - 1) % 89)
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let body: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [3, 5, 7, 9, 11, 13, 15, 17, 19];
    let s: i32 = body
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| -w * d as i32)
        .sum();
    let check = 11 + ((s - 1).rem_euclid(89)) as u32;
    let check_str = format!("{:02}", check);
    let body_str: String = body.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}", check_str, body_str)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let body = &digits[2..];
    let weights = [3i32, 5, 7, 9, 11, 13, 15, 17, 19];
    let s: i32 = body
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| -w * d as i32)
        .sum();
    let expected = 11 + ((s - 1).rem_euclid(89)) as u32;
    let actual = digits[0] as u32 * 10 + digits[1] as u32;
    expected == actual
}
