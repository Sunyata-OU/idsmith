use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    (0..6).map(|_| (b'0' + rng.gen_range(0..=9u8)) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace("FO", "").replace(' ', "");
    clean.len() == 6 && clean.chars().all(|c| c.is_ascii_digit())
}
