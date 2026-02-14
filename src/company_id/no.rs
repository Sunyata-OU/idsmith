use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        let weights = [3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let r = 11 - (sum % 11);
        if r == 10 {
            continue;
        }
        let check = if r == 11 { 0 } else { r };
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("NO{}{}{}MVA", s, check, ""); // Fix formatting
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [3, 2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let mut r = 11 - (sum % 11);
    if r == 11 {
        r = 0;
    }
    r == digits[8] as u32
}
