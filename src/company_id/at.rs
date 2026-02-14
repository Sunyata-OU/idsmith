use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let weights = [1, 2, 1, 2, 1, 2, 1];
    let mut sum: u32 = 0;
    for (i, &d) in digits.iter().enumerate() {
        let mut product = d as u32 * weights[i];
        if product > 9 {
            product = (product / 10) + (product % 10);
        }
        sum += product;
    }
    let check = (10 - (sum + 4) % 10) % 10;
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("ATU{}{}", s, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("ATU") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 8 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [1, 2, 1, 2, 1, 2, 1];
    let mut sum: u32 = 0;
    for (i, &d) in digits[..7].iter().enumerate() {
        let mut product = d as u32 * weights[i];
        if product > 9 {
            product = (product / 10) + (product % 10);
        }
        sum += product;
    }
    let check = (10 - (sum + 4) % 10) % 10;
    check == digits[7] as u32
}
