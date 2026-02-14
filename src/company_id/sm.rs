use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let lownumbers = [
        2, 4, 6, 7, 8, 9, 10, 11, 13, 16, 18, 19, 20, 21, 25, 26, 30, 32, 33, 35, 36, 37, 38, 39,
        40, 42, 45, 47, 49, 51, 52, 55, 56, 57, 58, 59, 61, 62, 64, 65, 66, 67, 68, 69, 70, 71, 72,
        73, 74, 75, 76, 79, 80, 81, 84, 85, 87, 88, 91, 92, 94, 95, 96, 97, 99,
    ];
    if rng.gen_bool(0.2) {
        format!("{}", lownumbers[rng.gen_range(0..lownumbers.len())])
    } else {
        format!("{}", rng.gen_range(100..100000))
    }
}

pub fn validate(code: &str) -> bool {
    let clean = code
        .replace(['.', ' '], "")
        .trim_start_matches('0')
        .to_string();
    if clean.is_empty() || clean.len() > 5 {
        return false;
    }
    if !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let val = clean.parse::<u32>().unwrap();
    if clean.len() < 3 {
        let lownumbers = [
            2, 4, 6, 7, 8, 9, 10, 11, 13, 16, 18, 19, 20, 21, 25, 26, 30, 32, 33, 35, 36, 37, 38,
            39, 40, 42, 45, 47, 49, 51, 52, 55, 56, 57, 58, 59, 61, 62, 64, 65, 66, 67, 68, 69, 70,
            71, 72, 73, 74, 75, 76, 79, 80, 81, 84, 85, 87, 88, 91, 92, 94, 95, 96, 97, 99,
        ];
        lownumbers.contains(&(val as i32))
    } else {
        true
    }
}
