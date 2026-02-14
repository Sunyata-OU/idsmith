use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = vec![rng.gen_range(1..=21)]; // simplified first two
    digits.push(rng.gen_range(0..=9)); // part of first two
    // wait first two is 01-22
    let first_two = rng.gen_range(1..=22);
    let mut res = vec![first_two / 10, first_two % 10];
    for _ in 0..6 { res.push(rng.gen_range(0..=9)); }
    res.extend_from_slice(&[0, 0, 1]); // branch
    
    let weights = [4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum: i32 = res.iter().zip(weights.iter()).map(|(&d, &w)| d as i32 * w as i32).sum();
    let check = (11 - (sum % 11)) % 11;
    res.push(check as u8);
    res.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.to_uppercase().replace("UY", "").chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 12 { return false; }
    let first_two = clean[..2].parse::<u8>().unwrap_or(0);
    if first_two < 1 || first_two > 22 { return false; }
    if &clean[2..8] == "000000" || &clean[8..11] != "001" { return false; }
    
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let weights = [4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum: i32 = digits[..11].iter().zip(weights.iter()).map(|(&d, &w)| d as i32 * w as i32).sum();
    let expected = (11 - (sum % 11)) % 11;
    expected as u8 == digits[11]
}
