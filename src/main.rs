use std::io::Write;

use clap::{Parser, Subcommand};
use rand::{thread_rng, Rng};

use idsmith::{bank_account, csv as csv_fmt, iban, personal_id};

#[derive(Parser)]
#[command(name = "idsmith")]
#[command(about = "Forge valid test IBANs, personal IDs, and bank accounts for 252 countries")]
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
    /// Generate random bank account numbers
    Account {
        /// Number of accounts to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., US, AU, GB). Random if omitted.
        #[arg(long)]
        country: Option<String>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
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
        Box::new(std::fs::File::create(path).unwrap_or_else(|e| {
            eprintln!("Cannot create {}: {}", path, e);
            std::process::exit(1);
        }))
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
                                csv_fmt::iban_row(
                                    &iban_code,
                                    &iban::format_iban(&iban_code),
                                    valid
                                )
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
        Commands::Account {
            count,
            country,
            list,
            csv,
        } => {
            let registry = bank_account::Registry::new();

            if list {
                println!(
                    "{:<6} {:<25} {:<30} {}",
                    "Code", "Country", "Format", "IBAN"
                );
                println!("{}", "-".repeat(70));
                for (code, country_name, format_name, has_iban) in registry.list_countries() {
                    println!(
                        "{:<6} {:<25} {:<30} {}",
                        code,
                        country_name,
                        format_name,
                        if has_iban { "Yes" } else { "No" }
                    );
                }
                return;
            }

            let opts = bank_account::GenOptions::default();

            let country = country.map(|c| c.to_uppercase());
            if let Some(ref c) = country {
                if !registry.is_supported(c) {
                    eprintln!("Unsupported country: {}", c);
                    std::process::exit(1);
                }
            }

            let mut out: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out {
                writeln!(w, "{}", csv_fmt::ACCOUNT_HEADER).unwrap();
            }

            for _ in 0..count {
                let result = match &country {
                    Some(c) => registry.generate(c, &opts, &mut rng).unwrap(),
                    None => {
                        let countries = registry.list_countries();
                        let pick = countries[rng.gen_range(0..countries.len())].0;
                        registry.generate(pick, &opts, &mut rng).unwrap()
                    }
                };

                if let Some(ref mut w) = out {
                    writeln!(w, "{}", csv_fmt::account_row(&result)).unwrap();
                } else {
                    println!(
                        "{} - {} - {}:",
                        result.country_code, result.country_name, result.format_name
                    );
                    let mut parts: Vec<String> = Vec::new();
                    if let Some(ref bank) = result.bank_code {
                        parts.push(format!("Bank: {}", bank));
                    }
                    if let Some(ref branch) = result.branch_code {
                        parts.push(format!("Branch: {}", branch));
                    }
                    parts.push(format!("Account: {}", result.account_number));
                    if let Some(ref check) = result.check_digits {
                        parts.push(format!("Check: {}", check));
                    }
                    if let Some(ref iban_code) = result.iban {
                        parts.push(format!("IBAN: {}", iban::format_iban(iban_code)));
                    }
                    parts.push(format!("Formatted: {}", result.formatted));
                    parts.push(format!("Raw: {}", result.raw));
                    parts.push(format!(
                        "valid: {}",
                        if result.valid { "True" } else { "False" }
                    ));
                    println!("  {}", parts.join(" | "));
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
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
                println!("{:<6} {:<25} {}", "Code", "Country", "ID Name");
                println!("{}", "-".repeat(55));
                for (code, country_name, name) in registry.list_countries() {
                    println!("{:<6} {:<25} {}", code, country_name, name);
                }
                return;
            }

            let country = country.to_uppercase();
            let name = match registry.name(&country) {
                Some(n) => n.to_string(),
                None => {
                    eprintln!("Unsupported country: {}", country);
                    let countries: Vec<_> = registry
                        .list_countries()
                        .iter()
                        .map(|(c, _, _)| *c)
                        .collect();
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
