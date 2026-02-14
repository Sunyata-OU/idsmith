use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    (0..8)
        .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
        .collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    clean.len() == 8
}
