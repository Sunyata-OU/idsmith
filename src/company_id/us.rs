use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefixes = [
        "10", "12", "20", "22", "30", "33", "35", "36", "40", "45", "55", "60", "90",
    ];
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let mut s = prefix.to_string();
    for _ in 0..7 {
        s.push((b'0' + rng.gen_range(0..10)) as char);
    }
    format!("{}-{}", &s[0..2], &s[2..])
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    clean.len() == 9
}
