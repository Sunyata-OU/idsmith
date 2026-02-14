use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut s = String::new();
    s.push((b'A' + rng.gen_range(0..13)) as char); // A-M
    for _ in 0..8 {
        s.push((b'0' + rng.gen_range(0..=9)) as char);
    }
    s.push((b'A' + rng.gen_range(0..26)) as char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace("AL", "").replace(' ', "");
    if clean.len() != 10 { return false; }
    let first = clean.chars().next().unwrap();
    let last = clean.chars().last().unwrap();
    first >= 'A' && first <= 'M' &&
    clean[1..9].chars().all(|c| c.is_ascii_digit()) &&
    last >= 'A' && last <= 'Z'
}
