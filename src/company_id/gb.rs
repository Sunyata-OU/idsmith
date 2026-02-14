use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let mut check = (sum % 97) as i32;
    check = 97 - check;
    if check < 0 {
        check += 97;
    }
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("GB{}{:02}", s, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [8, 7, 6, 5, 4, 3, 2];
    let sum: u32 = digits[..7]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let mut check = (sum % 97) as i32;
    check = 97 - check;
    if check < 0 {
        check += 97;
    }
    let actual_check: i32 = clean[7..9].parse().unwrap_or(-1);
    check == actual_check
}
