use crate::bank_account::checksum::luhn_check_digit;
use rand::Rng;

const FIRST_CHARS: &[u8] = b"ABCDEFGHJNPQRSUVW";
const CHECK_LETTERS: &[u8; 10] = b"JABCDEFGHI";

/// CIF (Spanish company tax number) - 9 chars: letter + 7 digits + check (digit or letter).
/// stdnum: Luhn on digits 1-7, check = both numeric and letter forms accepted.
pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let letter = FIRST_CHARS[rng.gen_range(0..FIRST_CHARS.len())] as char;
    let digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    let check = luhn_check_digit(&digits);
    // Use letter check for certain entity types, numeric for others
    let check_char = if b"NPQRSW".contains(&(letter as u8)) {
        CHECK_LETTERS[check as usize] as char
    } else {
        (b'0' + check) as char
    };
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("{}{}{}", letter, s, check_char)
}

pub fn validate(code: &str) -> bool {
    let clean: String = code.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if clean.len() != 9 {
        return false;
    }
    let bytes = clean.as_bytes();
    let first = bytes[0];
    if !FIRST_CHARS.contains(&first) {
        return false;
    }
    // Middle 7 must be digits
    let mid = &clean[1..8];
    if !mid.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = mid.bytes().map(|b| b - b'0').collect();
    let check = luhn_check_digit(&digits);
    let last = bytes[8];
    // Accept either numeric or letter check digit
    last == b'0' + check || last == CHECK_LETTERS[check as usize]
}
