use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    if digits[0] == 0 {
        digits[0] = 1;
    }

    let mut calc_digits = vec![0];
    calc_digits.extend_from_slice(&digits);

    let w1 = [3, 2, 7, 6, 5, 4, 3, 2];
    let sum: i32 = calc_digits
        .iter()
        .zip(w1.iter())
        .map(|(&d, &w)| d as i32 * w)
        .sum();
    let mut rem = (-sum).rem_euclid(11);

    if rem == 10 {
        let w2 = [7, 4, 3, 2, 5, 2, 7, 6];
        let sum2: i32 = calc_digits
            .iter()
            .zip(w2.iter())
            .map(|(&d, &w)| d as i32 * w)
            .sum();
        rem = (-sum2).rem_euclid(11);
        if rem == 10 {
            return "49098576".to_string();
        }
    }

    digits.push(rem as u8);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() < 8 || clean.len() > 9 {
        return false;
    }
    true
}
