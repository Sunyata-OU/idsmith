use std::io::Write;

use clap::{Parser, Subcommand};
use rand::thread_rng;

use eu_test_data_generator::{csv as csv_fmt, iban, personal_id};

#[derive(Parser)]
#[command(name = "eu-test-data-generator")]
#[command(about = "Generate valid test IBANs and personal ID codes for European countries")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate random IBANs
    Iban {
        /// Country code (e.g., EE, DE). Random if omitted.
        country: Option<String>,
        /// Number of IBANs to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
    },
    /// Generate random personal ID codes
    Id {
        /// Number of IDs to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code
        #[arg(long, default_value = "EE")]
        country: String,
        /// Gender (m or f)
        #[arg(long)]
        gender: Option<String>,
        /// Year of birth
        #[arg(long)]
        year: Option<u16>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
    },
}

fn csv_writer(path: &str) -> Box<dyn Write> {
    if path == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(
            std::fs::File::create(path).unwrap_or_else(|e| {
                eprintln!("Cannot create {}: {}", path, e);
                std::process::exit(1);
            }),
        )
    }
}

fn main() {
    let cli = Cli::parse();
    let mut rng = thread_rng();

    match cli.command {
        Commands::Iban {
            country,
            count,
            csv,
        } => {
            // Handle case where user passes just a number (e.g., `iban 3`)
            // clap parses it as country="3", count=1
            let (actual_country, actual_count) = match &country {
                Some(c) if c.chars().all(|ch| ch.is_ascii_digit()) => {
                    (None, c.parse::<u32>().unwrap_or(1))
                }
                _ => (country.as_deref(), count),
            };

            let mut out: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out {
                writeln!(w, "{}", csv_fmt::IBAN_HEADER).unwrap();
            }

            for _ in 0..actual_count {
                match iban::generate_iban(actual_country, &mut rng) {
                    Ok(iban_code) => {
                        let valid = iban::validate_iban(&iban_code);
                        if let Some(ref mut w) = out {
                            writeln!(
                                w,
                                "{}",
                                csv_fmt::iban_row(&iban_code, &iban::format_iban(&iban_code), valid)
                            )
                            .unwrap();
                        } else {
                            println!(
                                "{}  (valid: {})",
                                iban::format_iban(&iban_code),
                                if valid { "True" } else { "False" }
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", actual_count, path);
                }
            }
        }
        Commands::Id {
            count,
            country,
            gender,
            year,
            list,
            csv,
        } => {
            let registry = personal_id::Registry::new();

            if list {
                println!("{:<6} {:<25}", "Code", "ID Name");
                println!("{}", "-".repeat(31));
                for (code, name) in registry.list_countries() {
                    println!("{:<6} {}", code, name);
                }
                return;
            }

            let country = country.to_uppercase();
            let name = match registry.name(&country) {
                Some(n) => n.to_string(),
                None => {
                    eprintln!("Unsupported country: {}", country);
                    let countries: Vec<_> =
                        registry.list_countries().iter().map(|(c, _)| *c).collect();
                    eprintln!("Supported: {}", countries.join(", "));
                    std::process::exit(1);
                }
            };

            let opts = personal_id::GenOptions {
                gender: personal_id::date::Gender::from_str_opt(gender.as_deref()),
                year,
            };

            let mut out: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out {
                writeln!(w, "{}", csv_fmt::ID_HEADER).unwrap();
            } else {
                println!("{} - {}:", country, name);
            }

            for _ in 0..count {
                let code = registry.generate(&country, &opts, &mut rng).unwrap();
                let parsed = registry.parse(&country, &code).unwrap();
                if let Some(ref mut w) = out {
                    writeln!(w, "{}", csv_fmt::id_row(&country, &name, &parsed)).unwrap();
                } else {
                    let mut parts = Vec::new();
                    if let Some(ref g) = parsed.gender {
                        parts.push(g.clone());
                    }
                    if let Some(ref dob) = parsed.dob {
                        parts.push(dob.clone());
                    }
                    parts.push(format!(
                        "valid: {}",
                        if parsed.valid { "True" } else { "False" }
                    ));
                    println!("  {}  ({})", parsed.code, parts.join(", "));
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
    }
}
