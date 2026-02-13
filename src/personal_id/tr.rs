use rand::Rng;

use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let mut d: Vec<u8> = vec![rng.gen_range(1..=9)];
    for _ in 0..8 {
        d.push(rng.gen_range(0..=9));
    }
    let d10 = (((d[0] as i32 + d[2] as i32 + d[4] as i32 + d[6] as i32 + d[8] as i32) * 7)
        - (d[1] as i32 + d[3] as i32 + d[5] as i32 + d[7] as i32))
        .rem_euclid(10) as u8;
    d.push(d10);
    let d11 = d.iter().map(|&x| x as u32).sum::<u32>() % 10;
    d.push(d11 as u8);
    d.iter().map(|x| (b'0' + x) as char).collect()
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) || code.starts_with('0') {
        return false;
    }
    let d: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let d10 = (((d[0] as i32 + d[2] as i32 + d[4] as i32 + d[6] as i32 + d[8] as i32) * 7)
        - (d[1] as i32 + d[3] as i32 + d[5] as i32 + d[7] as i32))
        .rem_euclid(10) as u8;
    let d11 = (d[..10].iter().map(|&x| x as u32).sum::<u32>() % 10) as u8;
    d[9] == d10 && d[10] == d11
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
