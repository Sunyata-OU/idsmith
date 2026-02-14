use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [7, 9, 8, 6, 5, 4, 3, 2];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let check = ((10 - (sum % 11)) % 9 + 1) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 {
        return false;
    }
    let whitelist = [
        "101581601",
        "101582245",
        "101595422",
        "101595785",
        "10233317",
        "131188691",
        "401007374",
    ];
    if whitelist.contains(&clean.as_str()) {
        return true;
    }

    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [7, 9, 8, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let expected = ((10 - (sum % 11)) % 9 + 1) as u8;
    expected == digits[8]
}
