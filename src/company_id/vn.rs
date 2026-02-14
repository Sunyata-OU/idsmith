use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9)).collect();
        if digits[2..9] == [0, 0, 0, 0, 0, 0, 0] {
            continue;
        }
        let weights = [31, 29, 23, 19, 17, 13, 7, 5, 3];
        let total: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let check = 10 - (total % 11);
        if check > 9 {
            continue;
        }
        digits.push(check as u8);
        return digits.iter().map(|d| (b'0' + d) as char).collect();
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 && clean.len() != 13 {
        return false;
    }
    if clean[2..9] == *"0000000" {
        return false;
    }
    let digits: Vec<u8> = clean[..10].bytes().map(|b| b - b'0').collect();
    let weights = [31, 29, 23, 19, 17, 13, 7, 5, 3];
    let total: u32 = digits[..9]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    let expected = 10 - (total % 11);
    expected == digits[9] as u32
}
