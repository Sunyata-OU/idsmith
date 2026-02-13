use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn from_str_opt(s: Option<&str>) -> Option<Gender> {
        match s {
            Some("m" | "M" | "male") => Some(Gender::Male),
            Some("f" | "F" | "female") => Some(Gender::Female),
            _ => None,
        }
    }

    pub fn resolve_or_random(opt: Option<Gender>, rng: &mut impl Rng) -> Gender {
        opt.unwrap_or_else(|| {
            if rng.gen_bool(0.5) {
                Gender::Male
            } else {
                Gender::Female
            }
        })
    }
}

pub fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

pub fn rand_date(rng: &mut impl Rng, min_year: u16, max_year: u16) -> (u16, u8, u8) {
    let y = rng.gen_range(min_year..=max_year);
    let m = rng.gen_range(1..=12u8);
    let d = rng.gen_range(1..=days_in_month(y, m));
    (y, m, d)
}

pub fn rand_date_with_year(rng: &mut impl Rng, year: u16) -> (u16, u8, u8) {
    let m = rng.gen_range(1..=12u8);
    let d = rng.gen_range(1..=days_in_month(year, m));
    (year, m, d)
}

pub fn resolve_date(rng: &mut impl Rng, year: Option<u16>) -> (u16, u8, u8) {
    match year {
        Some(y) => rand_date_with_year(rng, y),
        None => rand_date(rng, 1940, 2005),
    }
}
