use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let digits: Vec<u8> = (0..rng.gen_range(5..11))
        .map(|_| rng.gen_range(0..=9))
        .collect();
    // weighted sum reversed weights 2,3,4...
    let mut sum: i32 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        sum += (i as i32 + 2) * d as i32;
    }
    let check = (-(sum) % 11).rem_euclid(11);
    let check_char = if check == 10 {
        'K'
    } else {
        (b'0' + check as u8) as char
    };
    let mut s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    s.push(check_char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code
        .to_uppercase()
        .replace([' ', '-'], "")
        .trim_start_matches('0')
        .to_string();
    if clean.len() < 2 || clean.len() > 12 {
        return false;
    }
    if !clean[..clean.len() - 1].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let digits: Vec<u8> = clean[..clean.len() - 1].bytes().map(|b| b - b'0').collect();
    let mut sum: i32 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        sum += (i as i32 + 2) * d as i32;
    }
    let expected = (-(sum) % 11).rem_euclid(11);
    let expected_char = if expected == 10 {
        'K'
    } else {
        (b'0' + expected as u8) as char
    };
    clean.ends_with(expected_char)
}
