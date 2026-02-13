use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const CHECK_CHARS: &[u8] = b"0123456789ABCDEFHJKLMNPRSTUVWXY";

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let sep = match y / 100 {
        18 => '+',
        19 => '-',
        20 => 'A',
        _ => '-',
    };
    let serial: u16 = loop {
        let s = rng.gen_range(2..=899u16);
        if (gender == Gender::Male && s % 2 == 1) || (gender == Gender::Female && s % 2 == 0) {
            break s;
        }
    };
    let num: u32 = format!("{:02}{:02}{:02}{:03}", d, m, y % 100, serial)
        .parse()
        .unwrap();
    let check = CHECK_CHARS[(num % 31) as usize] as char;
    format!(
        "{:02}{:02}{:02}{}{:03}{}",
        d,
        m,
        y % 100,
        sep,
        serial,
        check
    )
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 {
        return false;
    }
    let num_str = format!("{}{}", &code[..6], &code[7..10]);
    match num_str.parse::<u32>() {
        Ok(num) => CHECK_CHARS[(num % 31) as usize] == code.as_bytes()[10],
        Err(_) => false,
    }
}

pub fn parse(code: &str) -> IdResult {
    let dd: u8 = code[0..2].parse().unwrap_or(0);
    let mm: u8 = code[2..4].parse().unwrap_or(0);
    let yy: u16 = code[4..6].parse().unwrap_or(0);
    let sep = code.as_bytes()[6] as char;
    let century: u16 = match sep {
        '+' => 1800,
        '-' => 1900,
        'A' | 'B' | 'C' => 2000,
        _ => 1900,
    };
    let serial: u16 = code[7..10].parse().unwrap_or(0);
    IdResult {
        code: code.to_string(),
        gender: Some(if serial % 2 == 1 { "male" } else { "female" }.to_string()),
        dob: Some(format!("{}-{:02}-{:02}", century + yy, mm, dd)),
        valid: validate(code),
    }
}
