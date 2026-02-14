use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let mut digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
        let mut checksum = 10;
        for &d in &digits {
            checksum = (checksum + d as u32) % 10;
            if checksum == 0 { checksum = 10; }
            checksum = (checksum * 2) % 11;
        }
        let check = (11 - checksum) % 10;
        digits.push(check as u8);
        return digits.iter().map(|d| (b'0' + d) as char).collect();
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 9 { return false; }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let mut checksum = 10;
    for &d in &digits[..8] {
        checksum = (checksum + d as u32) % 10;
        if checksum == 0 { checksum = 10; }
        checksum = (checksum * 2) % 11;
    }
    let expected = (11 - checksum) % 10;
    expected == digits[8] as u32
}
