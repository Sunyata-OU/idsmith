use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let first = [4, 5, 9][rng.gen_range(0..3)];
        let mut digits: Vec<u8> = vec![first];
        for _ in 0..10 {
            digits.push(rng.gen_range(0..=9));
        }
        let weights = [9, 1, 4, 8, 3, 10, 2, 5, 7, 6, 1];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        if (sum % 11) == 3 {
            let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            return format!("LV{}", s);
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("LV") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 11 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if &clean[0..1] > "3" {
        let weights = [9, 1, 4, 8, 3, 10, 2, 5, 7, 6, 1];
        let sum: u32 = clean
            .bytes()
            .zip(weights.iter())
            .map(|(b, &w)| (b - b'0') as u32 * w as u32)
            .sum();
        (sum % 11) == 3
    } else {
        let weights = [10, 5, 8, 4, 2, 1, 6, 3, 7, 9];
        let sum: u32 = clean
            .bytes()
            .take(10)
            .zip(weights.iter())
            .map(|(b, &w)| (b - b'0') as u32 * w as u32)
            .sum();
        let check = (1 + sum) % 11 % 10;
        check == (clean.as_bytes()[10] - b'0') as u32
    }
}
