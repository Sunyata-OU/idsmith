use idsmith::{personal_ids, tax_ids};
use std::time::Instant;

fn main() {
    let iterations = 100_000;

    // Personal ID - US SSN
    let ssn = "123-45-6789";
    let start = Instant::now();
    for _ in 0..iterations {
        personal_ids().validate("US", ssn);
    }
    let duration = start.elapsed();
    println!(
        "Personal ID (US SSN): {:.2} ops/sec",
        iterations as f64 / duration.as_secs_f64()
    );

    // Tax ID - India PAN
    let pan = "ABCDE1234F";
    let start = Instant::now();
    for _ in 0..iterations {
        tax_ids().validate("IN", pan);
    }
    let duration = start.elapsed();
    println!(
        "Tax ID (India PAN): {:.2} ops/sec",
        iterations as f64 / duration.as_secs_f64()
    );
}
