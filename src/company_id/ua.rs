use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let mut weights = [1, 2, 3, 4, 5, 6, 7];
    if digits[0] >= 3 && digits[0] <= 5 { weights = [7, 1, 2, 3, 4, 5, 6]; }
    
    let mut sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
    let mut check = (sum % 11) as u8;
    if check >= 10 {
        let weights2 = weights.iter().map(|w| w + 2).collect::<Vec<_>>();
        sum = digits.iter().zip(weights2.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
        check = (sum % 11 % 10) as u8;
    }
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 8 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let mut weights = [1, 2, 3, 4, 5, 6, 7];
    if digits[0] >= 3 && digits[0] <= 5 { weights = [7, 1, 2, 3, 4, 5, 6]; }
    
    let mut sum: u32 = digits[..7].iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
    let mut expected = (sum % 11) as u8;
    if expected >= 10 {
        let weights2 = weights.iter().map(|w| w + 2).collect::<Vec<_>>();
        sum = digits[..7].iter().zip(weights2.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
        expected = (sum % 11 % 10) as u8;
    }
    expected == digits[7]
}
