use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut code = String::new();
    for _ in 0..3 {
        code.push((b'A' + rng.gen_range(0..26)) as char);
    }
    code.push_str("900101");
    let alphabet = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for _ in 0..3 {
        let idx = rng.gen_range(0..alphabet.len());
        code.push(alphabet.chars().nth(idx).unwrap());
    }
    code
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| !"- _".contains(*c)).collect();
    if clean.len() != 12 && clean.len() != 13 {
        return false;
    }
    if clean.len() == 12 {
        clean[0..3].chars().all(|c| c.is_alphabetic())
            && clean[3..9].chars().all(|c| c.is_ascii_digit())
    } else {
        clean[0..4].chars().all(|c| c.is_alphabetic())
            && clean[4..10].chars().all(|c| c.is_ascii_digit())
    }
}
