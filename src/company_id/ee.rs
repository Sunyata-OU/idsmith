use rand::Rng;

/// Registrikood (Estonian organisation registration code) - 8 digits.
/// First digit must be 1, 7, 8, or 9.
/// Uses same check digit algorithm as Isikukood:
///   weights ((i%9)+1) for i=0..6, with fallback (((i+2)%9)+1) if first gives 10.
fn calc_check_digit(digits: &[u8]) -> u8 {
    let check: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| ((i as u32 % 9) + 1) * d as u32)
        .sum::<u32>()
        % 11;
    if check == 10 {
        let check2: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| (((i as u32 + 2) % 9) + 1) * d as u32)
            .sum::<u32>()
            % 11;
        (check2 % 10) as u8
    } else {
        check as u8
    }
}

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let first = *[1u8, 7, 8, 9].get(rng.gen_range(0..4)).unwrap();
    let mut digits: Vec<u8> = vec![first];
    for _ in 1..7 {
        digits.push(rng.gen_range(0..=9));
    }
    let check = calc_check_digit(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 8 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    if !matches!(digits[0], 1 | 7 | 8 | 9) {
        return false;
    }
    let expected = calc_check_digit(&digits[..7]);
    expected == digits[7]
}
