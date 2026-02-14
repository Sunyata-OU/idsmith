use rand::Rng;

use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let area = loop {
        let a = rng.gen_range(1..=899u16);
        if a != 666 {
            break a;
        }
    };
    let group = rng.gen_range(1..=99u8);
    let serial = rng.gen_range(1..=9999u16);
    format!("{:03}{:02}{:04}", area, group, serial)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    if clean.len() != 9 || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let area: u16 = clean[0..3].parse().unwrap_or(0);
    let group: u8 = clean[3..5].parse().unwrap_or(0);
    let serial: u16 = clean[5..9].parse().unwrap_or(0);
    area >= 1 && area <= 899 && area != 666 && group >= 1 && serial >= 1
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| *c != '-').collect();
    IdResult {
        code: if clean.len() == 9 {
            format!("{}-{}-{}", &clean[0..3], &clean[3..5], &clean[5..9])
        } else {
            clean
        },
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
