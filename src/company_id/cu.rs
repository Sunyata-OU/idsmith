use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let year = rng.gen_range(0..=99);
    let month = rng.gen_range(1..=12);
    let day = rng.gen_range(1..=28);
    let century = rng.gen_range(0..=5); // 1900s
    format!("{:02}{:02}{:02}{}{:04}", year, month, day, century, rng.gen_range(0..10000))
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 { return false; }
    // Logic: YYMMDD + century_code + serial
    // We just check length and numeric as in stdnum.cu.ni
    true
}
