use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefix = "ACDEFGLOPU".chars().nth(rng.gen_range(0..10)).unwrap();
    let mut s = prefix.to_string();
    let num = match prefix {
        'F' => rng.gen_range(0..700000),
        'A' | 'L' => rng.gen_range(700000..800000),
        _ => rng.gen_range(0..1000000),
    };
    s.push_str(&format!("{:06}", num));
    s.push((b'A' + rng.gen_range(0..26u8)) as char);
    s
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace([' ', '-', '.'], "");
    if clean.len() != 8 {
        return false;
    }
    let first = clean.chars().next().unwrap();
    let last = clean.chars().last().unwrap();
    if !first.is_ascii_alphabetic() || !last.is_ascii_alphabetic() {
        return false;
    }
    if !clean[1..7].chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !"ACDEFGLOPU".contains(first) {
        return false;
    }

    let mid = &clean[1..7];
    if first == 'F' && mid > "699999" {
        return false;
    }
    if (first == 'A' || first == 'L') && !(mid > "699999" && mid < "800000") {
        return false;
    }

    true
}
