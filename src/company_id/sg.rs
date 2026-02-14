use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let format_choice = rng.gen_range(0..3);
    match format_choice {
        0 => {
            // Business (ROB): 8 digits + check letter
            let digits: Vec<u8> = (0..8).map(|_| rng.gen_range(0..=9)).collect();
            let weights = [10, 4, 9, 3, 8, 2, 7, 1];
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum();
            let check_char = "XMKECAWLJDB".chars().nth((sum % 11) as usize).unwrap();
            let mut s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            s.push(check_char);
            s
        }
        1 => {
            // Local Company (ROC): 9 digits + check letter (year + 5 digits)
            let year = rng.gen_range(1900..=2025);
            let mut digits: Vec<u8> = year.to_string().bytes().map(|b| b - b'0').collect();
            for _ in 0..5 {
                digits.push(rng.gen_range(0..=9));
            }
            let weights = [10, 8, 6, 4, 9, 7, 5, 3, 1];
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum();
            let check_char = "ZKCMDNERGWH".chars().nth((sum % 11) as usize).unwrap();
            let mut s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            s.push(check_char);
            s
        }
        _ => {
            // Others: T/S/R + yy + type + 4 digits + check
            let prefix = ["R", "S", "T"][rng.gen_range(0..3)];
            let year = rng.gen_range(0..=25);
            let entity_type = [
                "CC", "CD", "CH", "CL", "CM", "CP", "CS", "CX", "DP", "FB", "FC", "FM", "FN", "GA",
                "GB", "GS", "HS", "LL", "LP", "MB", "MC", "MD", "MH", "MM", "MQ", "NB", "NR", "PA",
                "PB", "PF", "RF", "RP", "SM", "SS", "TC", "TU", "VH", "XL",
            ][rng.gen_range(0..38)];
            let mut s = format!("{}{:02}{}", prefix, year, entity_type);
            for _ in 0..4 {
                s.push((b'0' + rng.gen_range(0..=9u8)) as char);
            }

            let alphabet = "ABCDEFGHJKLMNPQRSTUVWX0123456789";
            let weights = [4, 3, 5, 3, 10, 2, 2, 5, 7];
            let mut sum: i32 = 0;
            for (i, c) in s.chars().enumerate() {
                sum += (alphabet.find(c).unwrap() as i32) * weights[i];
            }
            let check_idx = ((sum - 5).rem_euclid(11)) as usize;
            s.push(alphabet.chars().nth(check_idx).unwrap());
            s
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean = code.to_uppercase();
    if clean.len() == 9 {
        if !clean[..8].chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let digits: Vec<u8> = clean[..8].bytes().map(|b| b - b'0').collect();
        let weights = [10, 4, 9, 3, 8, 2, 7, 1];
        let sum: u32 = digits
            .iter()
            .zip(weights.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        "XMKECAWLJDB".chars().nth((sum % 11) as usize) == Some(clean.chars().last().unwrap())
    } else if clean.len() == 10 {
        if clean.chars().next().unwrap().is_ascii_digit() {
            let digits: Vec<u8> = clean[..9].bytes().map(|b| b - b'0').collect();
            let weights = [10, 8, 6, 4, 9, 7, 5, 3, 1];
            let sum: u32 = digits
                .iter()
                .zip(weights.iter())
                .map(|(&d, &w)| d as u32 * w as u32)
                .sum();
            "ZKCMDNERGWH".chars().nth((sum % 11) as usize) == Some(clean.chars().last().unwrap())
        } else {
            if !["R", "S", "T"].contains(&&clean[0..1]) {
                return false;
            }
            let alphabet = "ABCDEFGHJKLMNPQRSTUVWX0123456789";
            let weights = [4, 3, 5, 3, 10, 2, 2, 5, 7];
            let mut sum: i32 = 0;
            for (i, c) in clean[..9].chars().enumerate() {
                if let Some(idx) = alphabet.find(c) {
                    sum += (idx as i32) * weights[i];
                } else {
                    return false;
                }
            }
            let check_idx = ((sum - 5).rem_euclid(11)) as usize;
            alphabet.chars().nth(check_idx) == Some(clean.chars().last().unwrap())
        }
    } else {
        false
    }
}
