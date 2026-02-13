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
