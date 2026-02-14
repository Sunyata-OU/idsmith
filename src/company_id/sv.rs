use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..13).map(|_| rng.gen_range(0..=9)).collect();
    digits[0] = *["0", "1", "9"][rng.gen_range(0..3)]
        .as_bytes()
        .first()
        .unwrap()
        - b'0';

    let check = if digits[10..13].iter().fold(0, |a, &b| a * 10 + b as u16) <= 100 {
        let weights = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        (sum % 11) % 10
    } else {
        let weights = [2, 7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        (110 - sum % 11) % 10
    };
    digits.push(check as u8);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 14 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    if ![0, 1, 9].contains(&digits[0]) {
        return false;
    }

    let expected = if digits[10..13].iter().fold(0, |a, &b| a * 10 + b as u16) <= 100 {
        let weights = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..13]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        (sum % 11) % 10
    } else {
        let weights = [2, 7, 6, 5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let sum: u32 = digits[..13]
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        (110 - sum % 11) % 10
    };
    expected as u8 == digits[13]
}
