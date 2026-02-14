use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [8, 7, 6, 5, 4, 3, 2];
    let sum: i32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as i32 * w)
        .sum();
    let check = ((-(sum) % 11).rem_euclid(11) % 10) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 8 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [8, 7, 6, 5, 4, 3, 2];
    let sum: i32 = digits[..7]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as i32 * w)
        .sum();
    let expected = ((-(sum) % 11).rem_euclid(11) % 10) as u8;
    expected == digits[7]
}
