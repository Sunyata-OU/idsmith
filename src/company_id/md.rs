use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = vec![rng.gen_range(1..=9)]; // registry
    let year = rng.gen_range(0..=25); // year code assigned
    digits.push((year / 10) as u8);
    digits.push((year % 10) as u8);
    for _ in 0..9 {
        digits.push(rng.gen_range(0..=9));
    }

    let weights = [7, 3, 1, 7, 3, 1, 7, 3, 1, 7, 3, 1];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let check = (sum % 10) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [7, 3, 1, 7, 3, 1, 7, 3, 1, 7, 3, 1];
    let sum: u32 = digits[..12]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let expected = (sum % 10) as u8;
    expected == digits[12]
}
