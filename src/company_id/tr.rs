use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..9).map(|_| rng.gen_range(0..=9)).collect();
    if digits[0] == 0 {
        digits[0] = 1;
    }

    let mut s = 0;
    for i in 1..=9 {
        let n = digits[9 - i] as i32;
        let c1 = (n + i as i32) % 10;
        if c1 != 0 {
            let mut c2 = (c1 * (1 << i)) % 9;
            if c2 == 0 {
                c2 = 9;
            }
            s += c2;
        }
    }
    let check = (10 - (s % 10)) % 10;
    digits.push(check as u8);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 10 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();

    let mut s = 0;
    for i in 1..=9 {
        let n = digits[9 - i] as i32;
        let c1 = (n + i as i32) % 10;
        if c1 != 0 {
            let mut c2 = (c1 * (1 << i)) % 9;
            if c2 == 0 {
                c2 = 9;
            }
            s += c2;
        }
    }
    let expected = (10 - (s % 10)) % 10;
    digits[9] == expected as u8
}
