use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum CharType {
    Numeric,
    Alpha,
    Alphanumeric,
}

#[derive(Debug, Clone, Copy)]
pub struct BbanField {
    pub length: u8,
    pub char_type: CharType,
}

const fn f(length: u8, char_type: CharType) -> BbanField {
    BbanField { length, char_type }
}

use CharType::{Alpha as A, Alphanumeric as C, Numeric as N};

const AL: &[BbanField] = &[f(3, N), f(4, N), f(1, N), f(16, C)];
const AT: &[BbanField] = &[f(5, N), f(11, N)];
const BE: &[BbanField] = &[f(3, N), f(7, N), f(2, N)];
const BA: &[BbanField] = &[f(3, N), f(3, N), f(8, N), f(2, N)];
const BG: &[BbanField] = &[f(4, A), f(6, N), f(8, C)];
const HR: &[BbanField] = &[f(7, N), f(10, N)];
const CY: &[BbanField] = &[f(3, N), f(5, N), f(16, C)];
const CZ: &[BbanField] = &[f(4, N), f(16, N)];
const DK: &[BbanField] = &[f(4, N), f(9, N), f(1, N)];
const EE: &[BbanField] = &[f(2, N), f(2, N), f(11, N), f(1, N)];
const FI: &[BbanField] = &[f(3, N), f(10, N), f(1, N)];
const FR: &[BbanField] = &[f(5, N), f(5, N), f(11, C), f(2, N)];
const DE: &[BbanField] = &[f(8, N), f(10, N)];
const GR: &[BbanField] = &[f(3, N), f(4, N), f(16, C)];
const HU: &[BbanField] = &[f(3, N), f(4, N), f(16, N), f(1, N)];
const IS: &[BbanField] = &[f(4, N), f(18, N)];
const IE: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];
const IT: &[BbanField] = &[f(1, A), f(5, N), f(5, N), f(12, C)];
const LV: &[BbanField] = &[f(4, A), f(13, C)];
const LI: &[BbanField] = &[f(5, N), f(12, C)];
const LT: &[BbanField] = &[f(5, N), f(11, N)];
const LU: &[BbanField] = &[f(3, N), f(13, C)];
const MT: &[BbanField] = &[f(4, A), f(5, N), f(18, C)];
const MD: &[BbanField] = &[f(2, C), f(18, C)];
const MC: &[BbanField] = &[f(5, N), f(5, N), f(11, C), f(2, N)];
const ME: &[BbanField] = &[f(3, N), f(13, N), f(2, N)];
const NL: &[BbanField] = &[f(4, A), f(10, N)];
const NO: &[BbanField] = &[f(4, N), f(6, N), f(1, N)];
const PL: &[BbanField] = &[f(3, N), f(4, N), f(1, N), f(16, N)];
const PT: &[BbanField] = &[f(4, N), f(4, N), f(11, N), f(2, N)];
const RO: &[BbanField] = &[f(4, A), f(16, C)];
const RS: &[BbanField] = &[f(3, N), f(13, N), f(2, N)];
const SK: &[BbanField] = &[f(4, N), f(16, N)];
const SI: &[BbanField] = &[f(2, N), f(3, N), f(8, N), f(2, N)];
const ES: &[BbanField] = &[f(4, N), f(4, N), f(2, N), f(10, N)];
const SE: &[BbanField] = &[f(3, N), f(16, N), f(1, N)];
const CH: &[BbanField] = &[f(5, N), f(12, C)];
const TR: &[BbanField] = &[f(5, N), f(1, N), f(16, C)];
const GB: &[BbanField] = &[f(4, A), f(6, N), f(8, N)];
const SA: &[BbanField] = &[f(2, N), f(18, C)];
const AE: &[BbanField] = &[f(3, N), f(16, N)];

// New countries (55 additions for full IBAN registry coverage)
const AD: &[BbanField] = &[f(8, N), f(12, C)];
const AX: &[BbanField] = &[f(14, N)];
const AZ: &[BbanField] = &[f(4, A), f(20, C)];
const BH: &[BbanField] = &[f(4, A), f(14, C)];
const BR: &[BbanField] = &[f(23, N), f(1, A), f(1, C)];
const BY: &[BbanField] = &[f(4, A), f(4, N), f(16, C)];
const CR: &[BbanField] = &[f(18, N)];
const DO: &[BbanField] = &[f(4, A), f(20, N)];
const EG: &[BbanField] = &[f(25, N)];
const FO: &[BbanField] = &[f(14, N)];
const GE: &[BbanField] = &[f(2, C), f(16, N)];
const GI: &[BbanField] = &[f(4, A), f(15, C)];
const GL: &[BbanField] = &[f(14, N)];
const GT: &[BbanField] = &[f(24, C)];
const IL: &[BbanField] = &[f(19, N)];
const IQ: &[BbanField] = &[f(4, A), f(15, N)];
const JO: &[BbanField] = &[f(4, A), f(4, N), f(18, C)];
const KW: &[BbanField] = &[f(4, A), f(22, C)];
const KZ: &[BbanField] = &[f(3, N), f(13, C)];
const LB: &[BbanField] = &[f(4, N), f(20, C)];
const LC: &[BbanField] = &[f(4, A), f(24, C)];
const LY: &[BbanField] = &[f(21, N)];
const MK: &[BbanField] = &[f(3, N), f(10, N), f(2, N)]; // all-numeric for mod97 check
const MN: &[BbanField] = &[f(16, N)];
const MR: &[BbanField] = &[f(23, N)];
const MU: &[BbanField] = &[f(4, A), f(19, N), f(3, A)];
const NI: &[BbanField] = &[f(4, A), f(20, N)];
const OM: &[BbanField] = &[f(3, N), f(16, C)];
const PK: &[BbanField] = &[f(4, C), f(16, N)];
const PS: &[BbanField] = &[f(4, C), f(21, N)];
const QA: &[BbanField] = &[f(4, A), f(21, C)];
const RU: &[BbanField] = &[f(14, N), f(15, C)];
const SC: &[BbanField] = &[f(4, A), f(20, N), f(3, A)];
const SD: &[BbanField] = &[f(14, N)];
const SM: &[BbanField] = &[f(1, A), f(10, N), f(12, C)]; // same as IT
const SO: &[BbanField] = &[f(19, N)];
const ST: &[BbanField] = &[f(21, N)];
const SV: &[BbanField] = &[f(4, A), f(20, N)];
const TL: &[BbanField] = &[f(19, N)];
const TN: &[BbanField] = &[f(20, N)];
const UA: &[BbanField] = &[f(6, N), f(19, C)];
const VA: &[BbanField] = &[f(18, N)];
const VG: &[BbanField] = &[f(4, C), f(16, N)];
const XK: &[BbanField] = &[f(16, N)];

struct CountryFormat {
    code: &'static str,
    fields: &'static [BbanField],
}

const ALL_FORMATS: &[CountryFormat] = &[
    CountryFormat {
        code: "AD",
        fields: AD,
    },
    CountryFormat {
        code: "AE",
        fields: AE,
    },
    CountryFormat {
        code: "AL",
        fields: AL,
    },
    CountryFormat {
        code: "AT",
        fields: AT,
    },
    CountryFormat {
        code: "AX",
        fields: AX,
    },
    CountryFormat {
        code: "AZ",
        fields: AZ,
    },
    CountryFormat {
        code: "BA",
        fields: BA,
    },
    CountryFormat {
        code: "BE",
        fields: BE,
    },
    CountryFormat {
        code: "BG",
        fields: BG,
    },
    CountryFormat {
        code: "BH",
        fields: BH,
    },
    CountryFormat {
        code: "BR",
        fields: BR,
    },
    CountryFormat {
        code: "BY",
        fields: BY,
    },
    CountryFormat {
        code: "CH",
        fields: CH,
    },
    CountryFormat {
        code: "CR",
        fields: CR,
    },
    CountryFormat {
        code: "CY",
        fields: CY,
    },
    CountryFormat {
        code: "CZ",
        fields: CZ,
    },
    CountryFormat {
        code: "DE",
        fields: DE,
    },
    CountryFormat {
        code: "DK",
        fields: DK,
    },
    CountryFormat {
        code: "DO",
        fields: DO,
    },
    CountryFormat {
        code: "EE",
        fields: EE,
    },
    CountryFormat {
        code: "EG",
        fields: EG,
    },
    CountryFormat {
        code: "ES",
        fields: ES,
    },
    CountryFormat {
        code: "FI",
        fields: FI,
    },
    CountryFormat {
        code: "FO",
        fields: FO,
    },
    CountryFormat {
        code: "FR",
        fields: FR,
    },
    CountryFormat {
        code: "GB",
        fields: GB,
    },
    CountryFormat {
        code: "GE",
        fields: GE,
    },
    CountryFormat {
        code: "GF",
        fields: FR,
    },
    CountryFormat {
        code: "GI",
        fields: GI,
    },
    CountryFormat {
        code: "GL",
        fields: GL,
    },
    CountryFormat {
        code: "GP",
        fields: FR,
    },
    CountryFormat {
        code: "GR",
        fields: GR,
    },
    CountryFormat {
        code: "GT",
        fields: GT,
    },
    CountryFormat {
        code: "HR",
        fields: HR,
    },
    CountryFormat {
        code: "HU",
        fields: HU,
    },
    CountryFormat {
        code: "IE",
        fields: IE,
    },
    CountryFormat {
        code: "IL",
        fields: IL,
    },
    CountryFormat {
        code: "IQ",
        fields: IQ,
    },
    CountryFormat {
        code: "IS",
        fields: IS,
    },
    CountryFormat {
        code: "IT",
        fields: IT,
    },
    CountryFormat {
        code: "JO",
        fields: JO,
    },
    CountryFormat {
        code: "KW",
        fields: KW,
    },
    CountryFormat {
        code: "KZ",
        fields: KZ,
    },
    CountryFormat {
        code: "LB",
        fields: LB,
    },
    CountryFormat {
        code: "LC",
        fields: LC,
    },
    CountryFormat {
        code: "LI",
        fields: LI,
    },
    CountryFormat {
        code: "LT",
        fields: LT,
    },
    CountryFormat {
        code: "LU",
        fields: LU,
    },
    CountryFormat {
        code: "LV",
        fields: LV,
    },
    CountryFormat {
        code: "LY",
        fields: LY,
    },
    CountryFormat {
        code: "MC",
        fields: MC,
    },
    CountryFormat {
        code: "MD",
        fields: MD,
    },
    CountryFormat {
        code: "ME",
        fields: ME,
    },
    CountryFormat {
        code: "MF",
        fields: FR,
    },
    CountryFormat {
        code: "MK",
        fields: MK,
    },
    CountryFormat {
        code: "MN",
        fields: MN,
    },
    CountryFormat {
        code: "MQ",
        fields: FR,
    },
    CountryFormat {
        code: "MR",
        fields: MR,
    },
    CountryFormat {
        code: "MT",
        fields: MT,
    },
    CountryFormat {
        code: "MU",
        fields: MU,
    },
    CountryFormat {
        code: "NC",
        fields: FR,
    },
    CountryFormat {
        code: "NI",
        fields: NI,
    },
    CountryFormat {
        code: "NL",
        fields: NL,
    },
    CountryFormat {
        code: "NO",
        fields: NO,
    },
    CountryFormat {
        code: "OM",
        fields: OM,
    },
    CountryFormat {
        code: "PF",
        fields: FR,
    },
    CountryFormat {
        code: "PK",
        fields: PK,
    },
    CountryFormat {
        code: "PL",
        fields: PL,
    },
    CountryFormat {
        code: "PM",
        fields: FR,
    },
    CountryFormat {
        code: "PS",
        fields: PS,
    },
    CountryFormat {
        code: "PT",
        fields: PT,
    },
    CountryFormat {
        code: "QA",
        fields: QA,
    },
    CountryFormat {
        code: "RE",
        fields: FR,
    },
    CountryFormat {
        code: "RO",
        fields: RO,
    },
    CountryFormat {
        code: "RS",
        fields: RS,
    },
    CountryFormat {
        code: "RU",
        fields: RU,
    },
    CountryFormat {
        code: "SA",
        fields: SA,
    },
    CountryFormat {
        code: "SC",
        fields: SC,
    },
    CountryFormat {
        code: "SD",
        fields: SD,
    },
    CountryFormat {
        code: "SE",
        fields: SE,
    },
    CountryFormat {
        code: "SI",
        fields: SI,
    },
    CountryFormat {
        code: "SK",
        fields: SK,
    },
    CountryFormat {
        code: "SM",
        fields: SM,
    },
    CountryFormat {
        code: "SO",
        fields: SO,
    },
    CountryFormat {
        code: "ST",
        fields: ST,
    },
    CountryFormat {
        code: "SV",
        fields: SV,
    },
    CountryFormat {
        code: "TF",
        fields: FR,
    },
    CountryFormat {
        code: "TL",
        fields: TL,
    },
    CountryFormat {
        code: "TN",
        fields: TN,
    },
    CountryFormat {
        code: "TR",
        fields: TR,
    },
    CountryFormat {
        code: "UA",
        fields: UA,
    },
    CountryFormat {
        code: "VA",
        fields: VA,
    },
    CountryFormat {
        code: "VG",
        fields: VG,
    },
    CountryFormat {
        code: "WF",
        fields: FR,
    },
    CountryFormat {
        code: "XK",
        fields: XK,
    },
    CountryFormat {
        code: "YT",
        fields: FR,
    },
];

fn get_format(country: &str) -> Option<&'static [BbanField]> {
    ALL_FORMATS
        .iter()
        .find(|f| f.code == country)
        .map(|f| f.fields)
}

pub fn supported_countries() -> Vec<&'static str> {
    ALL_FORMATS.iter().map(|f| f.code).collect()
}

fn random_chars(rng: &mut impl Rng, length: u8, char_type: CharType) -> String {
    let charset: &[u8] = match char_type {
        CharType::Numeric => b"0123456789",
        CharType::Alpha => b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        CharType::Alphanumeric => b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    };
    (0..length)
        .map(|_| charset[rng.gen_range(0..charset.len())] as char)
        .collect()
}

fn random_digits(rng: &mut impl Rng, length: u8) -> String {
    random_chars(rng, length, CharType::Numeric)
}

// =============================================================================
// BBAN national checksum helpers
// =============================================================================

/// Iterative mod-97 on a numeric string (handles arbitrary length)
fn bban_mod97(s: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in s.chars() {
        remainder = remainder * 10 + ch.to_digit(10).unwrap_or(0) as u64;
        remainder %= 97;
    }
    remainder
}

/// French BBAN letter-to-digit substitution for mod-97 check
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

/// Fix BBAN check digits for countries that ibantools validates internally.
/// Works on a Vec<u8> to avoid borrow issues, then converts back to String.
fn fix_bban_checksums(country: &str, bban: &mut String, rng: &mut impl Rng) {
    let mut b: Vec<u8> = bban.bytes().collect();

    match country {
        "NO" => {
            let weights: &[u8] = &[5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
            loop {
                // NO uses: remainder == 0 ? 0 : 11 - remainder, reject if >= 10
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
                // check == 10, regenerate account digits
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
        if nr % 10 != 0 {
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

fn generate_bban(country: &str, rng: &mut impl Rng) -> Option<String> {
    let fields = get_format(country)?;
    let mut bban = String::new();
    for field in fields {
        bban.push_str(&random_chars(rng, field.length, field.char_type));
    }
    fix_bban_checksums(country, &mut bban, rng);
    Some(bban)
}

fn letter_to_digits(ch: char) -> String {
    if ch.is_ascii_uppercase() {
        format!("{}", ch as u32 - 'A' as u32 + 10)
    } else {
        ch.to_string()
    }
}

fn iban_mod97(numeric_str: &str) -> u64 {
    let mut remainder: u64 = 0;
    for ch in numeric_str.chars() {
        remainder = remainder * 10 + ch.to_digit(10).unwrap() as u64;
        remainder %= 97;
    }
    remainder
}

fn calculate_check_digits(country_code: &str, bban: &str) -> String {
    let mut rearranged = String::new();
    for ch in bban.chars() {
        rearranged.push_str(&letter_to_digits(ch));
    }
    for ch in country_code.chars() {
        rearranged.push_str(&letter_to_digits(ch));
    }
    rearranged.push_str("00");
    let remainder = iban_mod97(&rearranged);
    format!("{:02}", 98 - remainder)
}

pub fn generate_iban(country: Option<&str>, rng: &mut impl Rng) -> Result<String, String> {
    let cc = match country {
        Some(c) => {
            let c = c.to_uppercase();
            if get_format(&c).is_none() {
                return Err(format!("Unsupported country: {}", c));
            }
            c
        }
        None => {
            let countries = supported_countries();
            countries[rng.gen_range(0..countries.len())].to_string()
        }
    };
    let bban = generate_bban(&cc, rng).unwrap();
    let check = calculate_check_digits(&cc, &bban);
    Ok(format!("{}{}{}", cc, check, bban))
}

pub fn format_iban(iban: &str) -> String {
    iban.chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn validate_iban(iban: &str) -> bool {
    let clean: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    if clean.len() < 4 {
        return false;
    }
    let rearranged: String = clean[4..].to_string() + &clean[..4];
    let numeric: String = rearranged.chars().map(letter_to_digits).collect();
    iban_mod97(&numeric) == 1
}
