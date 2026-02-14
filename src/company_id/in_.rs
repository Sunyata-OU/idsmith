use rand::Rng;

const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn luhn_mod36_checksum(code: &str) -> u32 {
    let n = 36u32;
    let mut sum = 0;
    for (i, c) in code.chars().rev().enumerate() {
        let val = ALPHABET.find(c).unwrap() as u32;
        if i % 2 == 1 {
            let doubled = val * 2;
            sum += (doubled / n) + (doubled % n);
        } else {
            sum += val;
        }
    }
    sum % n
}

pub fn generate(rng: &mut rand::rngs::ThreadRng) -> String {
    let states = [
        "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
        "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
        "31", "32", "33", "34", "35", "36", "37",
    ];
    let state = states[rng.gen_range(0..states.len())];

    let mut code = state.to_string();
    // PAN: 5 letters, 4 digits, 1 letter
    for _ in 0..3 {
        code.push((b'A' + rng.gen_range(0..26)) as char);
    }
    let entities = b"CPHFATBLJG";
    code.push(entities[rng.gen_range(0..entities.len())] as char);
    code.push((b'A' + rng.gen_range(0..26)) as char);
    for _ in 0..4 {
        code.push((b'0' + rng.gen_range(0..10)) as char);
    }
    code.push((b'A' + rng.gen_range(0..26)) as char);

    // 13th char: non-zero alphanumeric
    let c13 = ALPHABET.chars().nth(rng.gen_range(1..36)).unwrap();
    code.push(c13);
    code.push('Z');

    let ck = luhn_mod36_checksum(&(code.clone() + "0"));
    let check_char = ALPHABET.chars().nth(((36 - ck) % 36) as usize).unwrap();
    code.push(check_char);
    code
}

pub fn validate(code: &str) -> bool {
    if code.len() != 15 {
        return false;
    }
    if !code.chars().all(|c| ALPHABET.contains(c)) {
        return false;
    }
    luhn_mod36_checksum(code) == 0
}
