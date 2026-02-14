use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefix = ["C", "G", "Q", "V"][rng.gen_range(0..4)];
    let digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9)).collect();
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| (i as u32 + 1) * d as u32)
        .sum();
    let check = (sum % 11) as u8;
    let check_char = if check == 10 {
        'X'
    } else {
        (b'0' + check) as char
    };
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}00{}{}", prefix, s, check_char)
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace(' ', "");
    if clean.len() != 11 {
        return false;
    }
    if !["P", "C", "G", "Q", "V"].contains(&&clean[0..1]) || &clean[1..3] != "00" {
        return false;
    }
    if !clean[3..10].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let digits: Vec<u8> = clean[1..10].bytes().map(|b| b - b'0').collect();
    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| (i as u32 + 1) * d as u32)
        .sum();
    let expected = (sum % 11) as u8;
    let expected_char = if expected == 10 {
        'X'
    } else {
        (b'0' + expected) as char
    };
    clean.ends_with(expected_char)
}
