use rand::Rng;

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let mut digits: Vec<u8> = (0..7).map(|_| rng.gen_range(0..=9)).collect();
    digits.push(1);
    let calc_check = |ds: &[u8], ws: &[u8]| -> u32 {
        ds.iter()
            .zip(ws.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum::<u32>()
            % 11
    };
    let w1 = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut r = calc_check(&digits, &w1);
    if r == 10 {
        let w2 = [3, 4, 5, 6, 7, 8, 9, 1];
        r = calc_check(&digits, &w2);
        if r == 10 {
            r = 0;
        }
    }
    digits.push(r as u8);
    let s: String = digits.iter().map(|d| (b'0' + d) as char).collect();
    format!("LT{}", s)
}

pub fn validate(code: &str) -> bool {
    let clean: String = if let Some(stripped) = code.strip_prefix("LT") {
        stripped.to_string()
    } else {
        code.chars().filter(|c| c.is_ascii_digit()).collect()
    };
    if (clean.len() != 9 && clean.len() != 12) || !clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u8> = clean.bytes().map(|b| b - b'0').collect();
    let len = digits.len();
    let calc_check = |ds: &[u8], ws: &[u8]| -> u32 {
        let sum: u32 = ds
            .iter()
            .zip(ws.iter())
            .map(|(&d, &w)| d as u32 * w as u32)
            .sum();
        sum % 11
    };
    let w1: Vec<u32> = (1..len as u32)
        .map(|i| if i > 9 { i - 9 } else { i })
        .collect();
    let mut r = calc_check(
        &digits[..len - 1],
        &w1.iter().map(|&w| w as u8).collect::<Vec<_>>(),
    );
    if r == 10 {
        let w2: Vec<u32> = (3..len as u32 + 2)
            .map(|i| if i > 9 { i - 9 } else { i })
            .collect();
        r = calc_check(
            &digits[..len - 1],
            &w2.iter().map(|&w| w as u8).collect::<Vec<_>>(),
        );
        if r == 10 {
            r = 0;
        }
    }
    r == digits[len - 1] as u32
}
