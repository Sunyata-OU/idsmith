use rand::Rng;

use super::checksum::iso7064_mod11_10;
use super::IdResult;

pub fn generate(_opts: &super::GenOptions, rng: &mut impl Rng) -> String {
    loop {
        let mut digits: Vec<u8> = (0..10).collect();
        if rng.gen_bool(0.8) {
            // One digit appears twice
            let remove = rng.gen_range(0..10usize);
            digits.remove(remove);
            let dup = rng.gen_range(0..9usize);
            digits.push(digits[dup]);
        } else {
            // One digit appears three times
            let mut idxs: Vec<usize> = (0..10).collect();
            let i1 = rng.gen_range(0..10usize);
            idxs.remove(i1);
            let i2 = rng.gen_range(0..9usize);
            idxs.remove(i2);
            digits = idxs.iter().map(|&i| i as u8).collect();
            let dup = digits[rng.gen_range(0..8usize)];
            digits.push(dup);
            digits.push(dup);
        }

        // Shuffle
        for i in (1..digits.len()).rev() {
            let j = rng.gen_range(0..=i);
            digits.swap(i, j);
        }

        if digits[0] == 0 {
            let nz: Vec<usize> = (1..10).filter(|&i| digits[i] != 0).collect();
            if nz.is_empty() {
                continue;
            }
            let j = nz[rng.gen_range(0..nz.len())];
            digits.swap(0, j);
        }

        let check = iso7064_mod11_10(&digits);
        let code: String = digits.iter().map(|d| (b'0' + d) as char).collect();
        return format!("{}{}", code, check);
    }
}

pub fn validate(code: &str) -> bool {
    if code.len() != 11 || !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = code[..10].bytes().map(|b| b - b'0').collect();
    iso7064_mod11_10(&digits) == code.as_bytes()[10] - b'0'
}

pub fn parse(code: &str) -> IdResult {
    IdResult {
        code: code.to_string(),
        gender: None,
        dob: None,
        valid: validate(code),
    }
}
