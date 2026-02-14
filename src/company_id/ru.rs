use rand::Rng;

/// OGRN (Russian Primary State Registration Number).
/// 13-digit: first digit != 0, check = int(first_12) % 11 % 10.
/// 15-digit (OGRNIP): first digit 3 or 4, check = int(first_14) % 13 % 10.
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    if rng.gen_bool(0.5) {
        generate_13(rng)
    } else {
        generate_15(rng)
    }
}

fn generate_13(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = vec![rng.gen_range(1..=9)];
    for _ in 1..12 {
        digits.push(rng.gen_range(0..=9));
    }
    let n: u64 = digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
    let check = (n % 11 % 10) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

fn generate_15(rng: &mut rand::rngs::ThreadRng) -> String {
    let first = if rng.gen_bool(0.5) { 3u8 } else { 4 };
    let mut digits: Vec<u8> = vec![first];
    for _ in 1..14 {
        digits.push(rng.gen_range(0..=9));
    }
    let n: u128 = digits.iter().fold(0u128, |acc, &d| acc * 10 + d as u128);
    let check = (n % 13 % 10) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() == 13 {
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        if digits[0] == 0 {
            return false;
        }
        let n: u64 = digits[..12].iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
        let expected = (n % 11 % 10) as u8;
        expected == digits[12]
    } else if clean.len() == 15 {
        let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
        if !matches!(digits[0], 3 | 4) {
            return false;
        }
        let n: u128 = digits[..14].iter().fold(0u128, |acc, &d| acc * 10 + d as u128);
        let expected = (n % 13 % 10) as u8;
        expected == digits[14]
    } else {
        false
    }
}
