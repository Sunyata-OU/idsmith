use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [8, 9, 4, 5, 6, 7, 8, 9];
    let sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
    let check = (sum % 11) as usize;
    let check_digit = "01234567891".chars().nth(check).unwrap() as u8 - b'0';
    digits.push(check_digit);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [8, 9, 4, 5, 6, 7, 8, 9];
    let sum: u32 = digits[..8].iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
    let check_idx = (sum % 11) as usize;
    let expected = "01234567891".chars().nth(check_idx).unwrap() as u8 - b'0';
    expected == digits[8]
}
