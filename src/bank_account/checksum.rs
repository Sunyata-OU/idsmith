/// ABA routing number check digit (US).
/// Weighted sum with pattern 3,7,1 repeating, mod 10.
pub fn aba_check_digit(digits: &[u8]) -> u8 {
    let weights = [3u8, 7, 1, 3, 7, 1, 3, 7];
    let sum: u32 = digits[..8]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum();
    ((10 - sum % 10) % 10) as u8
}

/// CLABE verification digit (Mexico).
/// Weighted 3,7,1 repeating, each product mod 10, sum mod 10.
pub fn clabe_check_digit(digits: &[u8]) -> u8 {
    let weights = [3u8, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7];
    let sum: u32 = digits[..17]
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| (d as u32 * w as u32) % 10)
        .sum();
    ((10 - sum % 10) % 10) as u8
}

/// Luhn algorithm check digit.
pub fn luhn_check_digit(digits: &[u8]) -> u8 {
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

/// Weighted mod-11 check digit (used by BR, NZ, GB).
/// Returns the remainder. Caller decides how to map it to a check digit.
pub fn weighted_mod11(digits: &[u8], weights: &[u8]) -> u32 {
    digits
        .iter()
        .zip(weights.iter())
        .map(|(&d, &w)| d as u32 * w as u32)
        .sum::<u32>()
        % 11
}
