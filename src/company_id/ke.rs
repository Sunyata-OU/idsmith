use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut s = String::from("P");
    for _ in 0..9 { s.push((b'0' + rng.gen_range(0..=9u8)) as char); }
    s.push((b'A' + rng.gen_range(0..26)) as char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace(' ', "").replace('-', "");
    if clean.len() != 11 { return false; }
    let first = clean.chars().next().unwrap();
    let last = clean.chars().last().unwrap();
    (first == 'A' || first == 'P') &&
    clean[1..10].chars().all(|c| c.is_ascii_digit()) &&
    last.is_ascii_alphabetic()
}
