use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let len = if rng.gen_bool(0.5) { 7 } else { 9 };
        let digits: Vec<u8> = (0..len).map(|_| rng.gen_range(0..=9)).collect();
        // NINEA is 7 or 9 digits with checksum
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        
        let padded = format!("{:0>9}", s);
        let weights = [1, 2, 1, 2, 1, 2, 1, 2, 1];
        let sum: u32 = padded.bytes().zip(weights.iter()).map(|(b, &w)| (b - b'0') as u32 * w as u32).sum();
        if sum.is_multiple_of(10) {
            return s;
        }
    }
}

pub fn validate(code: &str) -> bool {
    let mut clean: String = code.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if clean.len() > 9 { clean = clean[..clean.len()-3].to_string(); }
    if clean.len() != 7 && clean.len() != 9 { return false; }
    if !clean.chars().all(|c| c.is_ascii_digit()) { return false; }
    
    let padded = format!("{:0>9}", clean);
    let weights = [1, 2, 1, 2, 1, 2, 1, 2, 1];
    let sum: u32 = padded.bytes().zip(weights.iter()).map(|(b, &w)| (b - b'0') as u32 * w as u32).sum();
    sum.is_multiple_of(10)
}
