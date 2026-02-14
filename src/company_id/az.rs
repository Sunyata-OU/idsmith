use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [4, 1, 8, 6, 2, 7, 5, 3];
    let sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
    let check = (sum % 11) as u8;
    digits.push(check);
    digits.push(rng.gen_range(1..=2)); // 1 for legal, 2 for natural
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let clean = if clean.len() == 9 { format!("0{}", clean) } else { clean };
    if clean.len() != 10 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    if digits[9] != 1 && digits[9] != 2 { return false; }
    let weights = [4, 1, 8, 6, 2, 7, 5, 3];
    let sum: u32 = digits[..8].iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
    let expected = (sum % 11) as u8;
    expected == digits[8]
}
