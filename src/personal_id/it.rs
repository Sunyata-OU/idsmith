use rand::Rng;

use super::date::{self, Gender};
use super::{GenOptions, IdResult};

const MONTHS: &[(u8, char)] = &[
    (1, 'A'),
    (2, 'B'),
    (3, 'C'),
    (4, 'D'),
    (5, 'E'),
    (6, 'H'),
    (7, 'L'),
    (8, 'M'),
    (9, 'P'),
    (10, 'R'),
    (11, 'S'),
    (12, 'T'),
];

fn month_to_letter(m: u8) -> char {
    MONTHS
        .iter()
        .find(|(mm, _)| *mm == m)
        .map(|(_, c)| *c)
        .unwrap_or('A')
}

fn letter_to_month(c: char) -> Option<u8> {
    MONTHS.iter().find(|(_, cc)| *cc == c).map(|(m, _)| *m)
}

// Odd-position values (1-indexed: positions 1,3,5,...)
fn odd_value(ch: u8) -> u32 {
    match ch {
        b'0' | b'A' => 1,
        b'1' | b'B' => 0,
        b'2' | b'C' => 5,
        b'3' | b'D' => 7,
        b'4' | b'E' => 9,
        b'5' | b'F' => 13,
        b'6' | b'G' => 15,
        b'7' | b'H' => 17,
        b'8' | b'I' => 19,
        b'9' | b'J' => 21,
        b'K' => 2,
        b'L' => 4,
        b'M' => 18,
        b'N' => 20,
        b'O' => 11,
        b'P' => 3,
        b'Q' => 6,
        b'R' => 8,
        b'S' => 12,
        b'T' => 14,
        b'U' => 16,
        b'V' => 10,
        b'W' => 22,
        b'X' => 25,
        b'Y' => 24,
        b'Z' => 23,
        _ => 0,
    }
}

fn even_value(ch: u8) -> u32 {
    match ch {
        b'0'..=b'9' => (ch - b'0') as u32,
        b'A'..=b'Z' => (ch - b'A') as u32,
        _ => 0,
    }
}

fn it_check(code15: &[u8]) -> char {
    let mut total: u32 = 0;
    for (i, &ch) in code15.iter().enumerate() {
        if (i + 1) % 2 == 1 {
            total += odd_value(ch);
        } else {
            total += even_value(ch);
        }
    }
    (b'A' + (total % 26) as u8) as char
}

pub fn generate(opts: &GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (y, m, d) = date::resolve_date(rng, opts.year);
    let consonants = b"BCDFGHJKLMNPQRSTVWXYZ";
    let surname: String = (0..3)
        .map(|_| consonants[rng.gen_range(0..consonants.len())] as char)
        .collect();
    let name: String = (0..3)
        .map(|_| consonants[rng.gen_range(0..consonants.len())] as char)
        .collect();
    let day_code = if gender == Gender::Female { d + 40 } else { d };
    let muni_letter = (b'A' + rng.gen_range(0..26u8)) as char;
    let muni_num: u16 = rng.gen_range(1..=999);
    let base = format!(
        "{}{}{:02}{}{:02}{}{:03}",
        surname,
        name,
        y % 100,
        month_to_letter(m),
        day_code,
        muni_letter,
        muni_num
    );
    let check = it_check(base.as_bytes());
    format!("{}{}", base, check)
}

pub fn validate(code: &str) -> bool {
    if code.len() != 16 {
        return false;
    }
    let upper = code.to_uppercase();
    it_check(upper[..15].as_bytes()) == upper.as_bytes()[15] as char
}

pub fn parse(code: &str) -> IdResult {
    let upper = code.to_uppercase();
    let yy: u16 = upper[6..8].parse().unwrap_or(0);
    let m_letter = upper.as_bytes()[8] as char;
    let dd: u8 = upper[9..11].parse().unwrap_or(0);
    let month = letter_to_month(m_letter);
    let gender_str = if dd > 40 { "female" } else { "male" };
    let actual_dd = if dd > 40 { dd - 40 } else { dd };
    let century: u16 = if yy <= 25 { 2000 } else { 1900 };
    let dob = month.map(|mm| format!("{}-{:02}-{:02}", century + yy, mm, actual_dd));
    IdResult {
        code: upper,
        gender: Some(gender_str.to_string()),
        dob,
        valid: validate(code),
    }
}
