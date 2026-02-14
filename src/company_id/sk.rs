use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let valid_d3 = [2, 3, 4, 7, 8, 9];
    loop {
        let mut digits: Vec<u8> = Vec::new();
        digits.push(2);
        digits.push(0);
        digits.push(valid_d3[rng.gen_range(0..valid_d3.len())]);
        for _ in 0..7 {
            digits.push(rng.gen_range(0..=9));
        }
        let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        let num: u64 = s.parse().unwrap();
        if num.is_multiple_of(11) {
            return format!("SK{}", s);
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("SK") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if clean.len() != 10 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let d3 = clean.as_bytes()[2] - b'0';
    if ![2, 3, 4, 7, 8, 9].contains(&d3) {
        return false;
    }
    let num: u64 = clean.parse().unwrap_or(1);
    num.is_multiple_of(11)
}
