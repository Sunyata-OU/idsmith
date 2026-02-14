use rand::Rng;

use super::{AccountResult, GenOptions};

/// Valid NZ bank codes with checksum algorithm and real branch numbers
/// from the NZ Payments registry. Banks with no known branches excluded.
static BANKS: &[(u8, char, &[u16])] = &[
    (
        1,
        'A',
        &[1, 142, 297, 450, 653, 795, 1108, 1146, 1184, 1822],
    ),
    (
        2,
        'A',
        &[18, 240, 390, 520, 644, 792, 918, 1215, 1252, 1295],
    ),
    (
        3,
        'A',
        &[31, 239, 442, 639, 814, 1313, 1387, 1544, 1704, 1768],
    ),
    (
        4,
        'A',
        &[2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023],
    ),
    (6, 'A', &[6, 172, 257, 350, 437, 529, 596, 738, 851, 939]),
    (
        8,
        'D',
        &[6501, 6504, 6515, 6523, 6533, 6543, 6557, 6567, 6581, 6589],
    ),
    (10, 'A', &[5165, 5166, 5167, 5168, 5169]),
    (
        11,
        'A',
        &[5000, 5314, 5462, 5832, 6424, 6919, 7278, 7446, 7920, 8381],
    ),
    (
        12,
        'A',
        &[3001, 3047, 3091, 3137, 3182, 3231, 3275, 3426, 3483, 3632],
    ),
    (
        13,
        'A',
        &[4901, 4903, 4905, 4908, 4910, 4913, 4915, 4917, 4926, 4928],
    ),
    (
        14,
        'A',
        &[4701, 4705, 4713, 4723, 4729, 4739, 4763, 4769, 4779, 4795],
    ),
    (
        15,
        'A',
        &[3941, 3944, 3948, 3951, 3955, 3958, 3969, 3972, 3976, 3979],
    ),
    (
        16,
        'A',
        &[4402, 4409, 4425, 4436, 4446, 4453, 4463, 4472, 4481, 4488],
    ),
    (
        17,
        'A',
        &[3331, 3361, 3365, 3369, 3373, 3377, 3381, 3385, 3389, 3393],
    ),
    (
        18,
        'A',
        &[3501, 3504, 3507, 3510, 3513, 3516, 3519, 3522, 3525, 3530],
    ),
    (
        19,
        'A',
        &[4617, 4618, 4620, 4621, 4624, 4626, 4629, 4631, 4635, 4647],
    ),
    (
        20,
        'A',
        &[4121, 4123, 4126, 4129, 4132, 4135, 4138, 4141, 4145, 4169],
    ),
    (
        21,
        'A',
        &[4801, 4804, 4808, 4811, 4815, 4819, 4822, 4826, 4829, 4895],
    ),
    (
        22,
        'A',
        &[4000, 4003, 4005, 4007, 4009, 4022, 4024, 4028, 4031, 4033],
    ),
    (
        23,
        'A',
        &[3700, 3703, 3716, 3730, 3736, 3750, 3758, 3765, 3784, 3792],
    ),
    (
        24,
        'A',
        &[4310, 4311, 4312, 4316, 4319, 4321, 4330, 4335, 4337, 4338],
    ),
    (
        25,
        'F',
        &[2500, 2510, 2525, 2531, 2537, 2543, 2548, 2554, 2559, 2565],
    ),
    (
        27,
        'A',
        &[3801, 3802, 3803, 3816, 3817, 3820, 3821, 3822, 3824, 3825],
    ),
    (
        30,
        'A',
        &[2901, 2902, 2904, 2906, 2908, 2911, 2912, 2922, 2932, 2940],
    ),
    (31, 'X', &[2825, 2826, 2827, 2828, 2829, 2840]),
    (
        38,
        'A',
        &[9000, 9050, 9100, 9150, 9200, 9250, 9300, 9350, 9400, 9450],
    ),
];

/// All valid bank codes for validation (including those without known branches).
static VALID_BANKS: &[(u8, char)] = &[
    (1, 'A'),
    (2, 'A'),
    (3, 'A'),
    (4, 'A'),
    (6, 'A'),
    (8, 'D'),
    (9, 'E'),
    (10, 'A'),
    (11, 'A'),
    (12, 'A'),
    (13, 'A'),
    (14, 'A'),
    (15, 'A'),
    (16, 'A'),
    (17, 'A'),
    (18, 'A'),
    (19, 'A'),
    (20, 'A'),
    (21, 'A'),
    (22, 'A'),
    (23, 'A'),
    (24, 'A'),
    (25, 'F'),
    (26, 'G'),
    (27, 'A'),
    (28, 'G'),
    (29, 'G'),
    (30, 'A'),
    (31, 'X'),
    (33, 'F'),
    (35, 'A'),
    (38, 'A'),
];

/// Weights per algorithm (16 elements: bank(2) + branch(4) + account(7) + suffix(3)).
fn weights(algo: char) -> &'static [u8; 16] {
    match algo {
        'A' => &[0, 0, 6, 3, 7, 9, 0, 10, 5, 8, 4, 2, 1, 0, 0, 0],
        'B' => &[0, 0, 0, 0, 0, 0, 0, 10, 5, 8, 4, 2, 1, 0, 0, 0],
        'D' => &[0, 0, 0, 0, 0, 0, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0],
        'E' => &[0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 4, 3, 2, 0, 0, 1],
        'F' => &[0, 0, 0, 0, 0, 0, 1, 7, 3, 1, 7, 3, 1, 0, 0, 0],
        'G' => &[0, 0, 0, 0, 0, 0, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1],
        'X' => &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        _ => &[0; 16],
    }
}

fn moduli(algo: char) -> (u32, u32) {
    match algo {
        'A' | 'B' | 'D' => (11, 11),
        'E' => (9, 11),
        'F' => (10, 10),
        'G' => (9, 10),
        _ => (1, 1),
    }
}

/// Calculate checksum for a 16-digit NZ account number.
fn calc_checksum(digits: &[u8; 16], algo: char) -> u32 {
    let w = weights(algo);
    let (mod1, mod2) = moduli(algo);
    let mut sum = 0u32;
    for i in 0..16 {
        let c = digits[i] as u32 * w[i] as u32;
        if c > mod1 {
            sum += c % mod1;
        } else {
            sum += c;
        }
    }
    sum % mod2
}

pub fn generate(_opts: &GenOptions, rng: &mut impl Rng) -> AccountResult {
    // Pick a random bank (with real branches) and its algorithm
    let &(bank, algo, branches) = &BANKS[rng.gen_range(0..BANKS.len())];

    // Pick a real branch
    let branch: u16 = branches[rng.gen_range(0..branches.len())];

    // Suffix: 3 digits, zero-padded
    let suffix: u16 = rng.gen_range(0..=99);

    let mut digits = [0u8; 16];
    digits[0] = bank / 10;
    digits[1] = bank % 10;
    digits[2] = (branch / 1000) as u8;
    digits[3] = ((branch / 100) % 10) as u8;
    digits[4] = ((branch / 10) % 10) as u8;
    digits[5] = (branch % 10) as u8;
    digits[13] = 0;
    digits[14] = (suffix / 10) as u8;
    digits[15] = (suffix % 10) as u8;

    // Fill account digits 6..11 randomly, solve for digit 12
    for d in &mut digits[6..12] {
        *d = rng.gen_range(0..=9);
    }

    // Determine effective algorithm same way validate() does:
    // Algorithm A becomes B for account number >= 0990000.
    // The first 6 account digits (positions 6..12) fully determine this:
    // msb6 >= 99000 means acct >= 990000 regardless of digit 12.
    let effective_algo = if algo == 'A' {
        let msb6: u32 = digits[6..12]
            .iter()
            .fold(0u32, |acc, &d| acc * 10 + d as u32);
        if msb6 >= 99_000 {
            'B'
        } else {
            'A'
        }
    } else {
        algo
    };

    let w = weights(effective_algo);
    if w[12] == 0 || effective_algo == 'X' {
        digits[12] = rng.gen_range(0..=9);
    } else {
        // Find digit d at position 12 that makes checksum == 0
        digits[12] = 0;
        let mut found = false;
        for d in 0..=9u8 {
            digits[12] = d;
            if calc_checksum(&digits, effective_algo) == 0 {
                found = true;
                break;
            }
        }
        if !found {
            // Fallback: try different digit 11
            for d11 in 0..=9u8 {
                digits[11] = d11;
                for d12 in 0..=9u8 {
                    digits[12] = d12;
                    if calc_checksum(&digits, effective_algo) == 0 {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
    }

    let num_str: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    let bank_str = &num_str[..2];
    let branch_str = &num_str[2..6];
    let account_str = &num_str[6..13];
    let suffix_str = &num_str[13..16];

    let formatted = format!("{}-{}-{}-{}", bank_str, branch_str, account_str, suffix_str);

    AccountResult {
        country_code: "NZ".into(),
        country_name: crate::countries::get_country_name("NZ").unwrap_or("Unknown").to_string(),
        format_name: "Bank + Branch + Account + Suffix".into(),
        bank_code: Some(bank_str.to_string()),
        branch_code: Some(branch_str.to_string()),
        account_number: account_str.to_string(),
        check_digits: None,
        formatted,
        raw: num_str,
        iban: None,
        valid: true,
    }
}

pub fn validate(raw: &str) -> bool {
    if raw.len() != 16 || !raw.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: [u8; 16] = std::array::from_fn(|i| raw.as_bytes()[i] - b'0');
    let bank = digits[0] * 10 + digits[1];
    let algo = VALID_BANKS
        .iter()
        .find(|&&(b, _)| b == bank)
        .map(|&(_, a)| a);
    let algo = match algo {
        Some(a) => a,
        None => return false,
    };
    // Algorithm A becomes B for account >= 0990000
    let effective = if algo == 'A' && &raw[6..13] >= "0990000" {
        'B'
    } else {
        algo
    };
    calc_checksum(&digits, effective) == 0
}

pub fn format(raw: &str) -> String {
    if raw.len() == 16 {
        format!(
            "{}-{}-{}-{}",
            &raw[..2],
            &raw[2..6],
            &raw[6..13],
            &raw[13..]
        )
    } else {
        raw.to_string()
    }
}
