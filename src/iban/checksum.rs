use rand::Rng;

use super::util::random_digits;

/// Iterative mod-97 on a numeric string (handles arbitrary length).
pub(crate) fn bban_mod97(s: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in s.chars() {
        remainder = remainder * 10 + ch.to_digit(10).unwrap_or(0) as u64;
        remainder %= 97;
    }
    remainder
}

/// French BBAN letter-to-digit substitution for mod-97 check.
fn fr_normalize_bban(bban: &str) -> String {
    bban.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let code = c.to_ascii_uppercase() as u8;
                match code {
                    b'A' | b'J' => '1',
                    b'B' | b'K' | b'S' => '2',
                    b'C' | b'L' | b'T' => '3',
                    b'D' | b'M' | b'U' => '4',
                    b'E' | b'N' | b'V' => '5',
                    b'F' | b'O' | b'W' => '6',
                    b'G' | b'P' | b'X' => '7',
                    b'H' | b'Q' | b'Y' => '8',
                    b'I' | b'R' | b'Z' => '9',
                    _ => '0',
                }
            } else {
                c
            }
        })
        .collect()
}

fn weighted_mod11_check_bytes(digits: &[u8], weights: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&b, &w)| (b - b'0') as u32 * w as u32)
        .sum();
    let remainder = sum % 11;
    if remainder == 0 {
        0
    } else if remainder == 1 {
        1
    } else {
        (11 - remainder) as u8
    }
}

fn weighted_mod10_check_bytes(digits: &[u8], weights: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .zip(weights.iter())
        .map(|(&b, &w)| (b - b'0') as u32 * w as u32)
        .sum();
    let remainder = sum % 10;
    if remainder == 0 {
        0
    } else {
        (10 - remainder) as u8
    }
}

fn mod_11_10_check_bytes(to_check: &[u8]) -> u8 {
    let mut nr: u32 = 10;
    for &byte in to_check {
        nr += (byte - b'0') as u32;
        if !nr.is_multiple_of(10) {
            nr %= 10;
        }
        nr = (nr * 2) % 11;
    }
    let result = 11u32.wrapping_sub(nr);
    if result == 10 {
        0
    } else {
        result as u8
    }
}

/// Fix BBAN check digits for countries that have national checksum algorithms.
pub(crate) fn fix_bban_checksums(country: &str, bban: &mut String, rng: &mut impl Rng) {
    let mut b: Vec<u8> = bban.bytes().collect();

    match country {
        "NO" => {
            let weights: &[u8] = &[5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
            loop {
                let sum: u32 = b[..10]
                    .iter()
                    .zip(weights.iter())
                    .map(|(&byte, &w)| (byte - b'0') as u32 * w as u32)
                    .sum();
                let remainder = sum % 11;
                let check = if remainder == 0 { 0 } else { 11 - remainder };
                if check <= 9 {
                    b[10] = b'0' + check as u8;
                    break;
                }
                let new_digits = random_digits(rng, 6);
                b[4..10].copy_from_slice(new_digits.as_bytes());
            }
        }

        "EE" => {
            let weights: &[u8] = &[7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7];
            let check = weighted_mod10_check_bytes(&b[2..15], weights);
            b[15] = b'0' + check;
        }

        "BE" => {
            let prefix: u64 = std::str::from_utf8(&b[..10]).unwrap().parse().unwrap_or(0);
            let check = prefix % 97;
            let check = if check == 0 { 97 } else { check };
            let s = format!("{:02}", check);
            b[10..12].copy_from_slice(s.as_bytes());
        }

        "BA" | "ME" | "MK" | "PT" | "RS" | "SI" => {
            let len = b.len();
            b[len - 2] = b'0';
            b[len - 1] = b'0';
            let s = std::str::from_utf8(&b).unwrap();
            let remainder = bban_mod97(s);
            let check = 98 - remainder;
            let cs = format!("{:02}", check);
            b[len - 2..len].copy_from_slice(cs.as_bytes());
        }

        "PL" => {
            let weights: &[u8] = &[3, 9, 7, 1, 3, 9, 7];
            let check = weighted_mod10_check_bytes(&b[..7], weights);
            b[7] = b'0' + check;
        }

        "ES" => {
            let weights_bank: &[u8] = &[4, 8, 5, 10, 9, 7, 3, 6];
            let weights_acct: &[u8] = &[1, 2, 4, 8, 5, 10, 9, 7, 3, 6];
            let check_bank = weighted_mod11_check_bytes(&b[..8], weights_bank);
            b[8] = b'0' + check_bank;
            let check_acct = weighted_mod11_check_bytes(&b[10..20], weights_acct);
            b[9] = b'0' + check_acct;
        }

        "HR" => {
            let check_bank = mod_11_10_check_bytes(&b[..6]);
            b[6] = b'0' + check_bank;
            let check_acct = mod_11_10_check_bytes(&b[7..16]);
            b[16] = b'0' + check_acct;
        }

        "CZ" | "SK" => {
            let weights_prefix: &[u8] = &[10, 5, 8, 4, 2, 1];
            let weights_suffix: &[u8] = &[6, 3, 7, 9, 10, 5, 8, 4, 2, 1];
            loop {
                let check_prefix = weighted_mod11_check_bytes(&b[4..9], weights_prefix);
                if check_prefix > 9 {
                    let new_digits = random_digits(rng, 5);
                    b[4..9].copy_from_slice(new_digits.as_bytes());
                    continue;
                }
                b[9] = b'0' + check_prefix;

                let check_suffix = weighted_mod11_check_bytes(&b[10..19], weights_suffix);
                if check_suffix > 9 {
                    let new_digits = random_digits(rng, 9);
                    b[10..19].copy_from_slice(new_digits.as_bytes());
                    continue;
                }
                b[19] = b'0' + check_suffix;
                break;
            }
        }

        "FR" | "GF" | "GP" | "MC" | "MF" | "MQ" | "NC" | "PF" | "PM" | "RE" | "TF" | "WF"
        | "YT" => {
            let len = b.len();
            b[len - 2] = b'0';
            b[len - 1] = b'0';
            let s = std::str::from_utf8(&b).unwrap();
            let normalized = fr_normalize_bban(s);
            let remainder = bban_mod97(&normalized);
            let check = if remainder == 0 { 0u64 } else { 97 - remainder };
            let cs = format!("{:02}", check);
            b[len - 2..len].copy_from_slice(cs.as_bytes());
        }

        "HU" => {
            let weights: &[u8] = &[9, 7, 3, 1, 9, 7, 3, 1, 9, 7, 3, 1, 9, 7, 3];
            let check_bb = weighted_mod10_check_bytes(&b[..7], &weights[..7]);
            b[7] = b'0' + check_bb;
            let check_acct = weighted_mod10_check_bytes(&b[8..23], weights);
            b[23] = b'0' + check_acct;
        }

        _ => {}
    }

    *bban = String::from_utf8(b).unwrap();
}
