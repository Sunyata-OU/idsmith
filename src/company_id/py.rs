use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..rng.gen_range(1..9)).map(|_| rng.gen_range(0..=9)).collect();
    if digits.is_empty() { digits.push(rng.gen_range(1..=9)); }
    
    let mut sum: i32 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        sum += (i as i32 + 2) * d as i32;
    }
    let check = ((-(sum) % 11).rem_euclid(11) % 10) as u8;
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() == 0 || clean.len() > 9 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let n = digits.len();
    let mut sum: i32 = 0;
    for (i, &d) in digits[..n-1].iter().rev().enumerate() {
        sum += (i as i32 + 2) * d as i32;
    }
    let expected = ((-(sum) % 11).rem_euclid(11) % 10) as u8;
    expected == digits[n-1]
}
