use rand::Rng;

use super::date::Gender;
use super::IdResult;

static STATES: &[&str] = &[
    "AS", "BC", "BS", "CC", "CL", "CM", "CS", "CH", "DF", "DG", "GT", "GR", "HG", "JC", "MC",
    "MN", "MS", "NT", "NL", "OC", "PL", "QT", "QR", "SP", "SL", "SR", "TC", "TS", "TL", "VZ",
    "YN", "ZS", "NE",
];

static BAD_WORDS: &[&str] = &[
    "BACA", "BAKA", "BUEI", "BUEY", "CACA", "CACO", "CAGA", "CAGO", "CAKA", "CAKO",
    "COGE", "COGI", "COJA", "COJE", "COJI", "COJO", "COLA", "CULO", "FALO", "FETO",
    "GETA", "GUEI", "GUEY", "JETA", "JOTO", "KACA", "KACO", "KAGA", "KAGO", "KAKA",
    "KAKO", "KOGE", "KOGI", "KOJA", "KOJE", "KOJI", "KOJO", "KOLA", "KULO", "LILO",
    "LOCA", "LOCO", "LOKA", "LOKO", "MAME", "MAMO", "MEAR", "MEAS", "MEON", "MIAR",
    "MION", "MOCO", "MOKO", "MULA", "MULO", "NACA", "NACO", "PEDA", "PEDO", "PENE",
    "PIPI", "PITO", "POPO", "PUTA", "PUTO", "QULO", "RATA", "ROBA", "ROBE", "ROBO",
    "RUIN", "SENO", "TETA", "VACA", "VAGA", "VAGO", "VAKA", "VUEI", "VUEY", "WUEI",
    "WUEY",
];

static VOWELS: &[u8] = b"AEIOU";
static CONSONANTS: &[u8] = b"BCDFGHJKLMNPQRSTVWXYZ";

fn char_value(c: u8) -> u32 {
    // Alphabet: 0-9 A-N & O-Z (& at index 24 represents Ñ)
    if c >= b'0' && c <= b'9' {
        (c - b'0') as u32
    } else if c >= b'A' && c <= b'N' {
        (c - b'A') as u32 + 10
    } else if c >= b'O' && c <= b'Z' {
        (c - b'A') as u32 + 11
    } else {
        0
    }
}

fn compute_check(chars: &[u8]) -> u8 {
    let sum: u32 = chars
        .iter()
        .enumerate()
        .map(|(i, &c)| char_value(c) * (18 - i as u32))
        .sum();
    let r = 10 - (sum % 10);
    (r % 10) as u8
}

fn random_letter(rng: &mut impl Rng) -> char {
    (b'A' + rng.gen_range(0..26u8)) as char
}

pub fn generate(opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    let gender = Gender::resolve_or_random(opts.gender, rng);
    let (year, month, day) = super::date::resolve_date(rng, opts.year);

    let gender_char = match gender {
        Gender::Male => 'H',
        Gender::Female => 'M',
    };
    let state = STATES[rng.gen_range(0..STATES.len())];

    loop {
        // Positions 0-3: surname1 initial, surname1 first vowel, surname2 initial, name initial
        let c1 = random_letter(rng);
        let v1 = VOWELS[rng.gen_range(0..VOWELS.len())] as char;
        let c2 = random_letter(rng);
        let c3 = random_letter(rng);

        let prefix = format!("{}{}{}{}", c1, v1, c2, c3);
        if BAD_WORDS.contains(&prefix.as_str()) {
            continue;
        }

        let ic1 = CONSONANTS[rng.gen_range(0..CONSONANTS.len())] as char;
        let ic2 = CONSONANTS[rng.gen_range(0..CONSONANTS.len())] as char;
        let ic3 = CONSONANTS[rng.gen_range(0..CONSONANTS.len())] as char;

        let homoclave = if year >= 2000 {
            (b'A' + rng.gen_range(0..26u8)) as char
        } else {
            (b'0' + rng.gen_range(0..=9u8)) as char
        };

        let base = format!(
            "{}{:02}{:02}{:02}{}{}{}{}{}{}",
            prefix,
            year % 100,
            month,
            day,
            gender_char,
            state,
            ic1,
            ic2,
            ic3,
            homoclave
        );

        let check = compute_check(base.as_bytes());
        return format!("{}{}", base, check);
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 18 {
        return false;
    }
    let bytes = code.as_bytes();
    if !bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
        return false;
    }
    // Gender at position 10
    let g = bytes[10];
    if g != b'H' && g != b'M' && g != b'X' {
        return false;
    }
    // DOB digits at positions 4-9
    if !bytes[4..10].iter().all(|b| b.is_ascii_digit()) {
        return false;
    }
    // Check digit
    let expected = compute_check(&bytes[..17]);
    (bytes[17] - b'0') == expected
}

pub fn parse(code: &str) -> IdResult {
    let (gender, dob) = if code.len() == 18 {
        let g = match code.as_bytes()[10] {
            b'H' => Some("male".to_string()),
            b'M' => Some("female".to_string()),
            _ => None,
        };
        let yy: u16 = code[4..6].parse().unwrap_or(0);
        let mm: u8 = code[6..8].parse().unwrap_or(0);
        let dd: u8 = code[8..10].parse().unwrap_or(0);
        // Infer century from homoclave character
        let h = code.as_bytes()[16];
        let year = if h.is_ascii_alphabetic() {
            2000 + yy
        } else {
            1900 + yy
        };
        let d = format!("{:04}-{:02}-{:02}", year, mm, dd);
        (g, Some(d))
    } else {
        (None, None)
    };

    IdResult {
        code: code.to_string(),
        gender,
        dob,
        valid: validate(code),
    }
}
