use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    // Monaco is like France but with 000 in middle
    // TVA format: FR + 2-digit check + 000 + 6 digits
    let siren = format!("000{:06}", rng.gen_range(0..1000000));
    let siren_val = siren.parse::<u64>().unwrap();
    let check = (12 + 3 * (siren_val % 97)) % 97;
    format!("FR{:02}{}", check, siren)
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase().replace(' ', "");
    if !clean.starts_with("FR") || clean.len() != 13 {
        return false;
    }
    if &clean[4..7] != "000" {
        return false;
    }
    crate::company_id::fr::validate(&clean)
}
