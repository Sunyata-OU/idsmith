use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = vec![0];
    for _ in 0..11 {
        digits.push(rng.gen_range(0..=9));
    }

    let mut s = 0;
    for (i, &d) in digits.iter().enumerate() {
        s += (13 - i as i32) * d as i32;
    }
    let r = s % 11;
    let check = (1 - r).rem_euclid(10);
    digits.push(check as u8);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 || !clean.starts_with('0') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let mut s = 0;
    for (i, &d) in digits[..12].iter().enumerate() {
        s += (13 - i as i32) * d as i32;
    }
    let r = s % 11;
    let expected = (1 - r).rem_euclid(10);
    expected == digits[12] as i32
}
