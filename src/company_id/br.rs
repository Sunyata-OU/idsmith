use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..12).map(|_| rng.gen_range(0..=9)).collect();
    let calc_dv = |slice: &[u8], weights: &[u8]| -> u8 {
        let sum: u32 = slice
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let r = (sum % 11) as u8;
        if r < 2 {
            0
        } else {
            11 - r
        }
    };
    let w1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    digits.push(calc_dv(&digits, &w1));
    let w2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    digits.push(calc_dv(&digits, &w2));
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 14 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let calc_dv = |slice: &[u8], weights: &[u8]| -> u8 {
        let sum: u32 = slice
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        let r = (sum % 11) as u8;
        if r < 2 {
            0
        } else {
            11 - r
        }
    };
    let w1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let w2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    digits[12] == calc_dv(&digits[..12], &w1) && digits[13] == calc_dv(&digits[..13], &w2)
}
