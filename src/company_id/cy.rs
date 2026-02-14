use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    if digits[0] == 1 && digits[1] == 2 {
        digits[1] = 3;
    }
    let translation = [1, 0, 5, 7, 9, 13, 15, 17, 19, 21];
    let mut sum = 0;
    for (i, &d) in digits.iter().enumerate() {
        let val = d as usize;
        if i % 2 == 0 {
            sum += translation[val];
        } else {
            sum += val;
        }
    }
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let check_char = alphabet.chars().nth(sum % 26).unwrap();
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("CY{}{}", s, check_char)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("CY") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
    };
    if clean.len() != 9 {
        return false;
    }
    if !clean[..8].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let translation = [1, 0, 5, 7, 9, 13, 15, 17, 19, 21];
    let mut sum = 0;
    for (i, c) in clean[..8].chars().enumerate() {
        let d = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            sum += translation[d];
        } else {
            sum += d;
        }
    }
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let check_char = alphabet.chars().nth(sum % 26).unwrap();
    clean.chars().last().unwrap().to_ascii_uppercase() == check_char
}
