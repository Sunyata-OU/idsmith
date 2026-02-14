use idsmith::iban;
use std::time::Instant;

fn main() {
    let iban = "DE47508562162522867909";
    let iterations = 100_000;

    let start = Instant::now();
    for _ in 0..iterations {
        iban::validate_iban(iban);
    }
    let duration = start.elapsed();

    println!(
        "idsmith (rust): {} iterations in {:.4} seconds",
        iterations,
        duration.as_secs_f64()
    );
    println!(
        "Throughput: {:.2} ops/sec",
        iterations as f64 / duration.as_secs_f64()
    );
}
