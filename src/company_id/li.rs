use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let len = rng.gen_range(4..=12);
    (0..len).map(|_| (b'0' + rng.gen_range(0..=9u8)) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean = code.replace('.', "").replace(' ', "").trim_start_matches('0').to_string();
    clean.len() >= 4 && clean.len() <= 12 && clean.chars().all(|c| c.is_ascii_digit())
}
