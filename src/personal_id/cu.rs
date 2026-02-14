use rand::Rng;

use super::date::Gender;
use super::IdResult;

/// Century digit: 9 → 1800s, 0-5 → 1900s, 6-8 → 2000s
fn century_digit(year: u16, rng: &mut impl Rng) -> u8 {
    match year {
        1800..=1899 => 9,
        1900..=1999 => rng.gen_range(0..=5),
        2000..=2099 => rng.gen_range(6..=8),
        _ => rng.gen_range(0..=5),
    }
}

fn year_from_century_digit(yy: u8, century: u8) -> u16 {
    let base = match century {
        9 => 1800,
        0..=5 => 1900,
        6..=8 => 2000,
        _ => 1900,
    };
    base + yy as u16
}

pub fn generate(opts: &super::GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    let year = opts.year.unwrap_or_else(|| rng.gen_range(1950..=2005));
    let month: u8 = rng.gen_range(1..=12);
    let day: u8 = rng.gen_range(1..=28);

    let cd = century_digit(year, rng);
    let yy = (year % 100) as u8;

    // Digits 7 (century) already chosen, digits 8-9 are serial, digit 10 encodes gender
    let serial: u8 = rng.gen_range(0..=99);
    let gender_digit: u8 = match opts.gender {
        Some(Gender::Male) => rng.gen_range(0..=4) * 2, // even → male
        Some(Gender::Female) => rng.gen_range(0..=4) * 2 + 1, // odd → female
        None => rng.gen_range(0..=9),
    };
    let last_digit: u8 = rng.gen_range(0..=9);

    format!(
        "{:02}{:02}{:02}{}{:02}{}{}",
        yy, month, day, cd, serial, gender_digit, last_digit
    )
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 11 {
        return false;
    }
    let bytes = clean.as_bytes();

    let yy = match clean[0..2].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let month = match clean[2..4].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let day = match clean[4..6].parse::<u8>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let century = bytes[6] - b'0';

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return false;
    }

    // Validate century digit
    if century > 9 {
        return false;
    }

    // Validate the date actually exists
    let year = year_from_century_digit(yy, century);
    let max_day = match month {
        2 => {
            if (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if day > max_day {
        return false;
    }

    true
}

pub fn parse(code: &str) -> IdResult {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    let valid = validate(code);
    let (dob, gender) = if valid && clean.len() == 11 {
        let yy = clean[0..2].parse::<u8>().unwrap_or(0);
        let month = clean[2..4].parse::<u8>().unwrap_or(0);
        let day = clean[4..6].parse::<u8>().unwrap_or(0);
        let century = clean.as_bytes()[6] - b'0';
        let year = year_from_century_digit(yy, century);
        let dob = format!("{:04}-{:02}-{:02}", year, month, day);
        let gender_digit = clean.as_bytes()[9] - b'0';
        let gender = if gender_digit.is_multiple_of(2) {
            "M"
        } else {
            "F"
        };
        (Some(dob), Some(gender.to_string()))
    } else {
        (None, None)
    };

    IdResult {
        country_code: "".to_string(),
        code: clean,
        gender,
        dob,
        valid,
    }
}
