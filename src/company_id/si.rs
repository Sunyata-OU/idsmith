use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let first = rng.gen_range(1..=9);
        let mut digits: Vec<u8> = vec![first];
        for _ in 0..6 {
            digits.push(rng.gen_range(0..=9));
        }

        let mut sum: u32 = 0;
        for (i, &d) in digits.iter().enumerate() {
            sum += (8 - i as u32) * d as u32;
        }
        let r = sum % 11;
        if r == 0 || r == 1 {
            continue;
        } // Both result in invalid SI numbers according to stdnum
        let check = 11 - r;
        // check will be 2..9
        digits.push(check as u8);
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("SI{}", s);
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("SI") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 8 || !clean.chars().all(|c| c.is_ascii_digit()) || clean.starts_with('0') {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();

    let mut sum: u32 = 0;
    for (i, &d) in digits[..7].iter().enumerate() {
        sum += (8 - i as u32) * d as u32;
    }
    let r = sum % 11;
    if r == 0 || r == 1 {
        return false;
    }
    let expected = 11 - r;
    expected == digits[7] as u32
}
