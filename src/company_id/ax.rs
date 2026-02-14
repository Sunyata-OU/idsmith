use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefix = "";
    let s: String = (0..10).map(|_| (b'0' + rng.gen_range(0..=9u8)) as char).collect();
    format!("{}{}", prefix, s)
}

pub fn validate(code: &str) -> bool {
    let clean_string: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let clean = &clean_string;
    if clean.len() != 10 { return false; }
    clean.chars().all(|c| c.is_ascii_digit())
}
