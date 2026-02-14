use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        // first char must be 1234567ABCEHKM
        // simplified to numeric 1-7 for now
        digits[0] = rng.gen_range(1..=7);
        let weights = [29, 23, 19, 17, 13, 7, 5, 3];
        let sum: u32 = digits.iter().zip(weights.iter()).map(|(&d, &w)| d as u32 * w as u32).sum();
        let check = sum % 11;
        if check <= 9 {
            digits.push(check as u8);
            return digits.iter().map(|d| (b'0' + d) as char).collect();
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace("УНП", "").replace("UNP", "").replace(' ', "");
    if clean.len() != 9 { return false; }
    if !clean[2..].chars().all(|c| c.is_ascii_digit()) { return false; }
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let weights = [29, 23, 19, 17, 13, 7, 5, 3];
    
    let mut digits: Vec<usize> = Vec::new();
    if clean[..2].chars().all(|c| c.is_ascii_digit()) {
        digits = clean.bytes().map(|b| (b - b'0') as usize).collect();
    } else {
        // Individual format: char + mapped char + digits
        // MA1953684
        let first = clean.chars().next().unwrap();
        let second = clean.chars().nth(1).unwrap();
        let map_alpha = "ABCEHKMOPT";
        if let Some(s_idx) = map_alpha.find(second) {
            digits.push(alphabet.find(first).unwrap_or(0));
            digits.push(s_idx);
            for c in clean[2..].chars() {
                digits.push((c as u8 - b'0') as usize);
            }
        } else { return false; }
    }
    
    if digits.len() != 9 { return false; }
    let sum: usize = digits[..8].iter().zip(weights.iter()).map(|(&d, &w)| d * (w as usize)).sum();
    let check = sum % 11;
    check <= 9 && check == digits[8]
}
