use rand::Rng;

use super::date;
use super::{GenOptions, IdResult};

const W: &[u8] = &[3, 7, 9, 5, 8, 4, 2, 1, 6];

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let date_part = format!("{:02}{:02}{:02}", d, m, y % 100);

    loop {
        let serial: u16 = rng.gen_range(100..=999);
        let mut all_digits: Vec<u8> = vec![
            (serial / 100) as u8,
            ((serial / 10) % 10) as u8,
            (serial % 10) as u8,
        ];
        for b in date_part.bytes() {
            all_digits.push(b - b'0');
        }
        let check: u32 = all_digits
            .iter()
            .zip(W.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum::<u32>()
            % 11;
        if check < 10 {
            return format!("{:03}{}{}", serial, check, date_part);
        }
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 10 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let d: Vec<u8> = code.bytes().map(|b| b - b'0').collect();
    let mut all_d: Vec<u8> = d[..3].to_vec();
    all_d.extend_from_slice(&d[4..]);
    let check: u32 = all_d
        .iter()
        .zip(W.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum::<u32>()
        % 11;
    check == d[3] as u32
}

pub fn parse(code: &str) -> IdResult {
    let dd: u8 = code[4..6].parse().unwrap_or(0);
    let mm: u8 = code[6..8].parse().unwrap_or(0);
    let yy: u16 = code[8..10].parse().unwrap_or(0);
    let century: u16 = if yy <= 25 { 2000 } else { 1900 };
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
