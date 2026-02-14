use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefixes = ["20", "23", "24", "27", "30", "33", "34", "50", "51", "55"];
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
    let map_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '9', '0'];
    let idx = (11 - rem) as usize;
    let check_char = map_chars[idx];

    let s: String = digits.iter().skip(2).map(|d| (b'0' + d) as char).collect();
    format!("{}-{}-{}", prefix, s, check_char)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 {
        return false;
    }
    let prefix = &clean[0..2];
    if !["20", "23", "24", "27", "30", "33", "34", "50", "51", "55"].contains(&prefix) {
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
    let map_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '9', '0'];
    let idx = (11 - rem) as usize;
    let expected = map_chars[idx] as u32 - '0' as u32;
    expected == digits[10] as u32
}
