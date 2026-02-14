use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut s = format!("{:07}", rng.gen_range(1..10000000));
    let alphabet = "ABCDEFGHJKLMNPQRSTUVWXYZ"; // simplified, exclusion list handled by random
    s.push(alphabet.chars().nth(rng.gen_range(0..alphabet.len())).unwrap());
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace(' ', "").replace('/', "").replace('.', "").replace('-', "");
    let clean = format!("{:0>8}", clean);
    if clean.len() != 8 && clean.len() != 13 { return false; }
    if !clean[..7].chars().all(|c| c.is_ascii_digit()) { return false; }
    let control = clean.chars().nth(7).unwrap();
    if "IOU".contains(control) || !control.is_ascii_alphabetic() { return false; }
    if clean.len() == 13 {
        if !"ABCDE".contains(clean.chars().nth(8).unwrap()) { return false; }
        if !"MCPNE".contains(clean.chars().nth(9).unwrap()) { return false; }
        if !clean[10..].chars().all(|c| c.is_ascii_digit()) { return false; }
    }
    true
}
