use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefixes = ["10", "15", "17", "20"];
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let mut digits: Vec<u8> = prefix.bytes().map(|b| b - b'0').collect();
    for _ in 0..8 {
        digits.push(rng.gen_range(0..=9));
    }

    let weights = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let rem = sum % 11;
    let check = (11 - rem) % 10;

    let s: String = digits.iter().skip(2).map(|d| (b'0' + d) as char).collect();
    format!("{}{}{}", prefix, s, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 {
        return false;
    }
    let prefix = &clean[0..2];
    if !["10", "15", "17", "20"].contains(&prefix) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..10]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let rem = sum % 11;
    let check = (11 - rem) % 10;
    check == digits[10] as u32
}
