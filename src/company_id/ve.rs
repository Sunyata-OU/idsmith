use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefix = ["V", "E", "J", "P", "G"][rng.gen_range(0..5)];
    let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
    let p_val = match prefix {
        "V" => 4,
        "E" => 8,
        "J" => 12,
        "P" => 16,
        "G" => 20,
        _ => 0,
    };
    let weights = [3, 2, 7, 6, 5, 4, 3, 2];
    let inner: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let sum: u32 = p_val + inner;
    let check = (11 - (sum % 11)) % 11;
    let check_char = if check >= 10 {
        '0'
    } else {
        (b'0' + check as u8) as char
    };
    let mut s = prefix.to_string();
    s.push_str(
        &digits
            .iter()
            .map(|d| (b'0' + d) as char)
            .collect::<String>(),
    );
    s.push(check_char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace(['-', ' '], "");
    if clean.len() != 10 {
        return false;
    }
    let prefix = &clean[0..1];
    let p_val = match prefix {
        "V" => 4,
        "E" => 8,
        "J" => 12,
        "P" => 16,
        "G" => 20,
        _ => return false,
    };
    if !clean[1..9].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean[1..9].bytes().map(|b| b - b'0').collect();
    let weights = [3, 2, 7, 6, 5, 4, 3, 2];
    let inner: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let sum: u32 = p_val + inner;
    let expected = (11 - (sum % 11)) % 11;
    let expected_char = if expected >= 10 {
        '0'
    } else {
        (b'0' + expected as u8) as char
    };
    clean.ends_with(expected_char)
}
