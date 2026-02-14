use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        let weights = [1, 2, 1, 2, 1, 2, 4, 1];
        let sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| {
            let p = d as u32 * w as u32;
            p / 10 + p % 10
        }).sum();
        if sum % 10 == 0 || (sum % 10 == 9 && digits[6] == 7) {
            return digits.iter().map(|d| (b'0' + d) as char).collect();
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 8 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [1, 2, 1, 2, 1, 2, 4, 1];
    let sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| {
        let p = d as u32 * w as u32;
        p / 10 + p % 10
    }).sum();
    sum % 10 == 0 || (sum % 10 == 9 && digits[6] == 7)
}
