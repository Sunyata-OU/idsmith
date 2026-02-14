use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let base = rng.gen_range(100_000..999_999);
    let check = base % 89;
    format!("LU{:06}{:02}", base, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("LU") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 8 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let num: u64 = clean.parse().unwrap_or(0);
    let base = num / 100;
    let check = num % 100;
    (base % 89) == check
}
