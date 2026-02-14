use idsmith::{iban, credit_cards, personal_ids, tax_ids, passports, driver_licenses};
use std::time::Instant;

fn bench<F>(name: &str, iterations: u32, f: F) 
where F: Fn() 
{
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();
    println!("{}: {:.2} ops/sec", name, iterations as f64 / duration.as_secs_f64());
}

fn main() {
    let iterations = 100_000;

    println!("idsmith (rust) benchmarks:");
    
    bench("IBAN (DE)", iterations, || {
        iban::validate_iban("DE47508562162522867909");
    });

    bench("Credit Card (Visa)", iterations, || {
        credit_cards().validate("4152839405126374");
    });

    bench("Personal ID (US SSN)", iterations, || {
        personal_ids().validate("US", "446-72-2445").unwrap();
    });

    bench("Tax ID (India PAN)", iterations, || {
        tax_ids().validate("IN", "ABCDE1234F");
    });

    bench("Passport (DE)", iterations, || {
        passports().validate("DE", "C01234567");
    });

    bench("Driver License (US)", iterations, || {
        driver_licenses().validate("US", "A123456789012");
    });
}
