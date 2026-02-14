use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let base: u64 = rng.gen_range(2_000_000..=9_999_999);
    let first_digit = 0u64;
    let full_base = first_digit * 10_000_000 + base;
    let check = 97 - (full_base % 97);
    format!("BE{:08}{:02}", full_base, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("BE") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 10 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let base: u64 = clean[..8].parse().unwrap_or(0);
    let check: u64 = clean[8..].parse().unwrap_or(0);
    97 - (base % 97) == check
}
