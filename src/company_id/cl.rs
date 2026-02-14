use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let len = rng.gen_range(7..=8); // Body length 7 or 8
    let digits: Vec<u8> = (0..len).map(|_| rng.gen_range(0..=9)).collect();

    let mut sum = 0;
    let mut multiplier = 2;
    for &d in digits.iter().rev() {
        sum += d as u32 * multiplier;
        multiplier += 1;
        if multiplier > 7 {
            multiplier = 2;
        }
    }
    let rem = 11 - (sum % 11);
    let check_char = match rem {
        11 => '0',
        10 => 'K',
        _ => (b'0' + rem as u8) as char,
    };

    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}-{}", s, check_char)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if clean.len() < 8 || clean.len() > 9 {
        return false;
    }
    let body = &clean[..clean.len() - 1];
    let check_char = clean.chars().last().unwrap().to_ascii_uppercase();
    if !body.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = body.bytes().map(|b| b - b'0').collect();
    let mut sum = 0;
    let mut multiplier = 2;
    for &d in digits.iter().rev() {
        sum += d as u32 * multiplier;
        multiplier += 1;
        if multiplier > 7 {
            multiplier = 2;
        }
    }
    let rem = 11 - (sum % 11);
    let expected = match rem {
        11 => '0',
        10 => 'K',
        _ => (b'0' + rem as u8) as char,
    };
    check_char == expected
}
