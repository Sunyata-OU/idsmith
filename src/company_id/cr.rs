use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let class = ["3", "4", "5"][rng.gen_range(0..3)];
    let type_val = match class {
        "3" => ["101", "102", "103", "104"][rng.gen_range(0..4)],
        "4" => "000",
        "5" => "001",
        _ => "000",
    };
    let mut s = format!("{}{}", class, type_val);
    for _ in 0..6 {
        s.push((b'0' + rng.gen_range(0..=9u8)) as char);
    }
    s
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 {
        return false;
    }
    let class = clean.chars().next().unwrap();
    let type_val = &clean[1..4];
    match class {
        '2' => ["100", "200", "300", "400"].contains(&type_val),
        '3' => [
            "002", "003", "004", "005", "006", "007", "008", "009", "010", "011", "012", "013",
            "014", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110",
        ]
        .contains(&type_val),
        '4' => type_val == "000",
        '5' => type_val == "001",
        _ => false,
    }
}
