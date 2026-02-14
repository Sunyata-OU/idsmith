use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        let weights = [9, 8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = sum % 11;
        if check < 10 {
            digits.push(check as u8);
            let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            return format!("NL{}B01", s);
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("NL") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
    };
    if clean.len() != 12 || !clean.contains('B') {
        return false;
    }
    let bsn = &clean[..9];
    if !bsn.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = bsn.bytes().map(|b| b - b'0').collect();
    let weights = [9, 8, 7, 6, 5, 4, 3, 2, -1];
    let sum: i32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as i32 * w)
        .sum();
    sum % 11 == 0
}
