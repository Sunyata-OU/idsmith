use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut s = String::from("100");
    for _ in 0..12 {
        s.push((b'0' + rng.gen_range(0..=9u8)) as char);
    }
    s
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    clean.len() == 15 && clean.starts_with("100")
}
