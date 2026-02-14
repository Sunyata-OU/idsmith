use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = vec![0];
    digits.push(rng.gen_range(1..=9)); // type
    digits.push(rng.gen_range(0..=9)); // type
    for _ in 0..6 {
        digits.push(rng.gen_range(0..=9));
    }

    // Luhn over first 9 (new 16-digit format with leading 0)
    let check = crate::personal_id::checksum::luhn_check(&digits);
    digits.push(check);
    for _ in 0..6 {
        digits.push(rng.gen_range(0..=9));
    }
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() == 15 {
        crate::personal_id::checksum::luhn_check(
            &clean[..8].bytes().map(|b| b - b'0').collect::<Vec<_>>(),
        ) == (clean.as_bytes()[8] - b'0')
    } else if clean.len() == 16 {
        if !clean.starts_with('0') {
            return crate::personal_id::id_::validate(&clean);
        }
        crate::personal_id::checksum::luhn_check(
            &clean[..9].bytes().map(|b| b - b'0').collect::<Vec<_>>(),
        ) == (clean.as_bytes()[9] - b'0')
    } else {
        false
    }
}
