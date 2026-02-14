use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut s = String::new();
    s.push((b'A' + rng.gen_range(0..26)) as char);
    for _ in 0..12 {
        s.push((b'0' + rng.gen_range(0..=9)) as char);
    }

    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum: u32 = 0;
    for (i, c) in s.chars().enumerate() {
        sum += (14 - i as u32) * alphabet.find(c).unwrap_or(0) as u32;
    }
    let check_idx = (17 - (sum % 17)) % 17;
    s.push(alphabet.chars().nth(check_idx as usize).unwrap());
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace(' ', "");
    if clean.len() != 14 {
        return false;
    }
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    if !clean.chars().next().unwrap().is_ascii_alphabetic() {
        return false;
    }

    let mut sum: u32 = 0;
    for (i, c) in clean[..13].chars().enumerate() {
        if let Some(idx) = alphabet.find(c) {
            sum += (14 - i as u32) * idx as u32;
        } else {
            return false;
        }
    }
    let expected_idx = (17 - (sum % 17)) % 17;
    clean.ends_with(alphabet.chars().nth(expected_idx as usize).unwrap())
}
