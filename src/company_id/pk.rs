use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let province = ["1", "2", "3", "4", "5", "6", "7"][rng.gen_range(0..7)];
    let mut s = format!("{}", province);
    for _ in 0..11 { s.push((b'0' + rng.gen_range(0..=9u8)) as char); }
    let last = (rng.gen_range(0..10));
    s.push((b'0' + last as u8) as char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 { return false; }
    let province = &clean[0..1];
    ["1", "2", "3", "4", "5", "6", "7"].contains(&province)
}
