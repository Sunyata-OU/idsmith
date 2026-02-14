use rand::Rng;

/// CI checksum (natural persons): weights (2,1) alternating, fold >9 by subtracting 9.
fn ci_checksum(digits: &[u8]) -> u32 {
    digits
        .iter()
        .enumerate()
        .map(|(i, &d)| {
            let w = if i % 2 == 0 { 2 } else { 1 };
            let v = d as u32 * w;
            if v > 9 {
                v - 9
            } else {
                v
            }
        })
        .sum::<u32>()
        % 10
}

/// Weighted checksum for public/juridical RUC types.
fn weighted_checksum(digits: &[u8], weights: &[u32]) -> u32 {
    digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum::<u32>()
        % 11
}

/// RUC (Ecuadorian company tax number) - 13 digits.
/// Type determined by 3rd digit: 0-5=natural, 6=public, 9=juridical.
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let province = rng.gen_range(1u8..=24);
        let kind = *[0u8, 6, 9].get(rng.gen_range(0..3)).unwrap();

        if kind < 6 {
            // Natural: 10-digit CI + 3-digit establishment (001+)
            let third = rng.gen_range(0u8..=5);
            let mut digits: Vec<u8> = vec![province / 10, province % 10, third];
            for _ in 3..9 {
                digits.push(rng.gen_range(0..=9));
            }
            // CI check digit: make ci_checksum of all 10 digits == 0
            let partial_sum = ci_checksum(&digits);
            let check = (10 - partial_sum) % 10;
            digits.push(check as u8);
            // Verify
            if ci_checksum(&digits) != 0 {
                continue;
            }
            // Establishment suffix 001-999
            let est = rng.gen_range(1u16..=999);
            let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            return format!("{}{:03}", s, est);
        } else if kind == 6 {
            // Public: weights (3,2,7,6,5,4,3,2), weighted_checksum of first 8 digits + check = 9 digits, must sum%11==0
            let mut digits: Vec<u8> = vec![province / 10, province % 10, 6];
            for _ in 3..8 {
                digits.push(rng.gen_range(0..=9));
            }
            let weights: [u32; 8] = [3, 2, 7, 6, 5, 4, 3, 2];
            let sum = weighted_checksum(&digits[..8], &weights);
            let check = if sum == 0 { 0 } else { 11 - sum };
            if check >= 10 {
                continue;
            }
            digits.push(check as u8);
            // Verify: first 9 digits with weights (3,2,7,6,5,4,3,2,1) should sum%11==0
            if weighted_checksum(&digits[..9], &[3, 2, 7, 6, 5, 4, 3, 2, 1]) != 0 {
                continue;
            }
            // Establishment suffix 0001-9999
            let est = rng.gen_range(1u16..=9999);
            let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            return format!("{}{:04}", s, est);
        } else {
            // Juridical (9): weights (4,3,2,7,6,5,4,3,2), weighted_checksum of first 9 digits + check = 10 digits
            let mut digits: Vec<u8> = vec![province / 10, province % 10, 9];
            for _ in 3..9 {
                digits.push(rng.gen_range(0..=9));
            }
            let weights: [u32; 9] = [4, 3, 2, 7, 6, 5, 4, 3, 2];
            let sum = weighted_checksum(&digits[..9], &weights);
            let check = if sum == 0 { 0 } else { 11 - sum };
            if check >= 10 {
                continue;
            }
            digits.push(check as u8);
            // Verify: first 10 digits with weights (4,3,2,7,6,5,4,3,2,1) should sum%11==0
            if weighted_checksum(&digits[..10], &[4, 3, 2, 7, 6, 5, 4, 3, 2, 1]) != 0 {
                continue;
            }
            // Establishment suffix 001-999
            let est = rng.gen_range(1u16..=999);
            let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
            return format!("{}{:03}", s, est);
        }
    }
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean.len() != 13 {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let province = digits[0] * 10 + digits[1];
    if !((1..=24).contains(&province) || province == 30 || province == 50) {
        return false;
    }

    let third = digits[2];
    if third <= 5 {
        // Natural: CI checksum on first 10 digits must be 0, establishment != 000
        ci_checksum(&digits[..10]) == 0 && clean[10..13] != *"000"
    } else if third == 6 {
        // Public: try public validation, fallback to natural
        let public_ok = weighted_checksum(&digits[..9], &[3, 2, 7, 6, 5, 4, 3, 2, 1]) == 0
            && clean[9..13] != *"0000";
        if public_ok {
            return true;
        }
        // Fallback: natural validation
        ci_checksum(&digits[..10]) == 0 && clean[10..13] != *"000"
    } else if third == 9 {
        // Juridical: try public first, then juridical
        let public_ok = weighted_checksum(&digits[..9], &[3, 2, 7, 6, 5, 4, 3, 2, 1]) == 0
            && clean[9..13] != *"0000";
        if public_ok {
            return true;
        }
        weighted_checksum(&digits[..10], &[4, 3, 2, 7, 6, 5, 4, 3, 2, 1]) == 0
            && clean[10..13] != *"000"
    } else {
        false
    }
}
