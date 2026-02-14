pub fn luhn_check(digits: &[u8]) -> u8 {
    let mut total: u32 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        let mut val = d as u32;
        if i % 2 == 0 {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        total += val;
    }
    ((10 - total % 10) % 10) as u8
}

pub fn iso7064_mod11_10(digits: &[u8]) -> u8 {
    let mut product: u32 = 10;
    for &d in digits {
        let mut s = (d as u32 + product) % 10;
        if s == 0 {
            s = 10;
        }
        product = (s * 2) % 11;
    }
    ((11 - product) % 10) as u8
}

pub fn weighted_check(digits: &[u8], weights: &[u8], modulus: u32) -> u32 {
    digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum::<u32>()
        % modulus
}

/// Verhoeff check digit algorithm using dihedral group D5.
/// Returns the check digit (0-9) that makes the full number valid.
pub fn verhoeff_check(digits: &[u8]) -> u8 {
    static D: [[u8; 10]; 10] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
        [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
        [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
        [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
        [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
        [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
        [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
        [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    ];
    static P: [[u8; 10]; 8] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
        [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
        [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
        [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
        [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
        [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
        [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
    ];
    static INV: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

    let mut c: u8 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        c = D[c as usize][P[(i + 1) % 8][d as usize] as usize];
    }
    INV[c as usize]
}

/// Verhoeff validation: returns true if the full number (including check digit) is valid.
pub fn verhoeff_validate(digits: &[u8]) -> bool {
    static D: [[u8; 10]; 10] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
        [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
        [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
        [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
        [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
        [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
        [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
        [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    ];
    static P: [[u8; 10]; 8] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
        [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
        [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
        [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
        [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
        [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
        [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
    ];

    let mut c: u8 = 0;
    for (i, &d) in digits.iter().rev().enumerate() {
        c = D[c as usize][P[i % 8][d as usize] as usize];
    }
    c == 0
}

/// ISO 7064 MOD 11-2 check character for Chinese Resident ID.
/// Returns '0'-'9' or 'X'.
pub fn iso7064_mod11_2(digits: &[u8]) -> char {
    static WEIGHTS: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    static CHECK_CHARS: &[u8] = b"10X98765432";
    let sum: u32 = digits
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&d, &w)| d as u32 * w)
        .sum();
    CHECK_CHARS[(sum % 11) as usize] as char
}
