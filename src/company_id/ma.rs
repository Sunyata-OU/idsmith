use rand::Rng;

/// ICE (Identifiant Commun de l'Entreprise, Morocco) - 15 digits.
/// stdnum: ISO 7064 mod 97-10, valid when int(number) % 97 == 0.
fn mod97(digits: &str) -> u32 {
    let mut rem: u64 = 0;
    for b in digits.bytes() {
        rem = (rem * 10 + (b - b'0') as u64) % 97;
    }
    rem as u32
}

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let body: String = (0..13)
            .map(|_| (b'0' + rng.gen_range(0..=9u8)) as char)
            .collect();
        let body_val = mod97(&body);
        // We need (body * 100 + check) % 97 == 0
        // check = (97 - (body_val * 100 % 97)) % 97
        let check = (97 - (body_val as u64 * 100 % 97)) % 97;
        let full = format!("{}{:02}", body, check);
        if full.len() == 15 {
            return full;
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 15 {
        return false;
    }
    mod97(&clean) == 0
}
