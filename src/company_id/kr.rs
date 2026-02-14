use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let head = rng.gen_range(101..=999);
    let mid = rng.gen_range(1..=99);
    let tail = rng.gen_range(1..=9999);
    let check = rng.gen_range(0..=9);
    format!("{:03}-{:02}-{:04}{}", head, mid, tail, check)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 {
        return false;
    }
    let head = &clean[0..3];
    let mid = &clean[3..5];
    let tail = &clean[5..9];
    if head < "101" {
        return false;
    }
    if mid == "00" {
        return false;
    }
    if tail == "0000" {
        return false;
    }
    true
}
