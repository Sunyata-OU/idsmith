use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = vec![rng.gen_range(1..=9)];
        for _ in 0..7 {
            digits.push(rng.gen_range(0..=9));
        }
        let weights = [2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..7]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let r = sum % 11;
        let check = if r == 0 { 0 } else { 11 - r };
        if check == 10 {
            continue;
        }
        if digits[7] != check as u8 {
            digits[7] = check as u8;
        }
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("DK{}", s);
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("DK") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 8 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if clean.starts_with('0') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [2, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..7]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let r = sum % 11;
    let expected = if r == 0 { 0 } else { 11 - r };
    if expected == 10 {
        return false;
    }
    expected == digits[7] as u32
}
