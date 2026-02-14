use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..12).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: i32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| d as i32 * w as i32).sum();
    let check = ((-(sum) % 11).rem_euclid(11) % 10) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.to_uppercase().replace("MK", "").replace("МК", "").chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: i32 = digits[..12].iter().zip(weights.iter()).map(|(&d, &w)| d as i32 * w as i32).sum();
    let expected = ((-(sum) % 11).rem_euclid(11) % 10) as u8;
    expected == digits[12]
}
