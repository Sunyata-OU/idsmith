use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let alphabet = "0123456789ABCDEFGHJKLMNPQRTUWXY";
    let mut code = String::new();
    code.push('9');
    code.push('1');
    for _ in 0..6 {
        code.push((b'0' + rng.gen_range(0..10)) as char);
    }
    for _ in 0..9 {
        let idx = rng.gen_range(0..alphabet.len());
        code.push(alphabet.chars().nth(idx).unwrap());
    }
    let weights = [
        1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28,
    ];
    let mut total = 0;
    for (i, c) in code.chars().enumerate() {
        let pos = alphabet.find(c).unwrap();
        total += pos * weights[i];
    }
    let check_idx = (31 - (total % 31)) % 31;
    code.push(alphabet.chars().nth(check_idx).unwrap());
    code
}

pub fn validate(code: &str) -> bool {
    let alphabet = "0123456789ABCDEFGHJKLMNPQRTUWXY";
    if code.len() != 18 {
        return false;
    }
    if !code[..8].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !code[8..].chars().all(|c| alphabet.contains(c)) {
        return false;
    }
    let weights = [
        1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28,
    ];
    let mut total = 0;
    for (i, c) in code[..17].chars().enumerate() {
        if let Some(pos) = alphabet.find(c) {
            total += pos * weights[i];
        } else {
            return false;
        }
    }
    let check_idx = (31 - (total % 31)) % 31;
    code.chars().nth(17) == alphabet.chars().nth(check_idx)
}
