use std::io::Write;

use clap::{Parser, Subcommand};
use rand::{thread_rng, Rng};

use idsmith::{
    bank_account, company_id, credit_card, csv as csv_fmt, driver_license, iban, lei, passport,
    personal_id, swift, tax_id, vat,
};

#[derive(Parser)]
#[command(name = "idsmith")]
#[command(
    about = "Validate and generate checksum-correct IBANs, personal IDs, bank accounts, and more"
)]
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
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
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
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
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
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random credit card numbers
    Card {
        /// Number of cards to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Brand (visa, mastercard, amex, discover, jcb, diners)
        #[arg(long)]
        brand: Option<String>,
        /// List all supported brands
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random SWIFT/BIC codes
    Swift {
        /// Number of codes to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., US, GB)
        #[arg(long)]
        country: Option<String>,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random Company/Business IDs (VAT, CIF, etc.)
    Company {
        /// Number of IDs to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., DE, GB, FR, IT, ES)
        #[arg(long)]
        country: Option<String>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random driver's license numbers
    License {
        /// Number of licenses to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., IN, US, GB)
        #[arg(long)]
        country: Option<String>,
        /// State code (for India: MH, DL, KA, etc.)
        #[arg(long)]
        state: Option<String>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random tax IDs (PAN, TIN, etc.)
    Tax {
        /// Number of tax IDs to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., IN, US, GB)
        #[arg(long)]
        country: Option<String>,
        /// Holder type (for India PAN: P, C, H, F, A, T, B, L, J, G)
        #[arg(long)]
        holder_type: Option<String>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random passport numbers
    Passport {
        /// Number of passports to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., IN, US, GB)
        #[arg(long)]
        country: Option<String>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random EU VAT numbers
    Vat {
        /// Number of VAT numbers to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., DE, FR, EL for Greece)
        #[arg(long)]
        country: Option<String>,
        /// List all supported countries
        #[arg(long)]
        list: bool,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Generate random LEI (Legal Entity Identifier) codes
    Lei {
        /// Number of LEIs to generate
        #[arg(default_value = "1")]
        count: u32,
        /// Country code (e.g., US, GB, DE)
        #[arg(long)]
        country: Option<String>,
        /// Export as CSV (optionally to a file path)
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        csv: Option<String>,
        /// Export as JSON (optionally to a file path)
        #[cfg(feature = "json")]
        #[arg(long, num_args = 0..=1, default_missing_value = "-")]
        json: Option<String>,
    },
    /// Validate an existing code
    Validate {
        /// Category (iban, account, id, card, swift, company)
        #[arg(index = 1)]
        category: String,
        /// Code to validate
        #[arg(index = 2)]
        code: String,
        /// Country code (required for most categories)
        #[arg(long)]
        country: Option<String>,
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
            json,
        } => {
            // Handle case where user passes just a number (e.g., `iban 3`)
            // clap parses it as country="3", count=1
            let (actual_country, actual_count) = match &country {
                Some(c) if c.chars().all(|ch| ch.is_ascii_digit()) => {
                    (None, c.parse::<u32>().unwrap_or(1))
                }
                _ => (country.as_deref(), count),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::IBAN_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..actual_count {
                match iban::generate_iban(actual_country, &mut rng) {
                    Ok(iban_code) => {
                        let valid = iban::validate_iban(&iban_code);
                        let formatted = iban::format_iban(&iban_code);

                        #[cfg(feature = "json")]
                        if json.is_some() {
                            json_results.push(iban::IbanResult {
                                country: iban_code[..2].to_string(),
                                iban: iban_code.clone(),
                                formatted: formatted.clone(),
                                valid,
                            });
                        }

                        if let Some(ref mut w) = out_csv {
                            writeln!(w, "{}", csv_fmt::iban_row(&iban_code, &formatted, valid))
                                .unwrap();
                        } else {
                            let mut print_it = true;
                            #[cfg(feature = "json")]
                            if json.is_some() {
                                print_it = false;
                            }

                            if print_it {
                                println!(
                                    "{}  (valid: {})",
                                    formatted,
                                    if valid { "True" } else { "False" }
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", actual_count, path);
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
            json,
        } => {
            let registry = bank_account::Registry::new();

            if list {
                println!("{:<6} {:<25} {:<30} IBAN", "Code", "Country", "Format");
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

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::ACCOUNT_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match &country {
                    Some(c) => registry.generate(c, &opts, &mut rng).unwrap(),
                    None => {
                        let countries = registry.list_countries();
                        let pick = countries[rng.gen_range(0..countries.len())].0;
                        registry.generate(pick, &opts, &mut rng).unwrap()
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::account_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
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
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
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
            json,
        } => {
            let registry = personal_id::Registry::new();

            if list {
                println!("{:<6} {:<25} ID Name", "Code", "Country");
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

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::ID_HEADER).unwrap();
            } else {
                let mut print_it = true;
                #[cfg(feature = "json")]
                if json.is_some() {
                    print_it = false;
                }
                if print_it {
                    println!("{} - {}:", country, name);
                }
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let code = registry.generate(&country, &opts, &mut rng).unwrap();
                let parsed = registry.parse(&country, &code).unwrap();

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(parsed.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::id_row(&country, &name, &parsed)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
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
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Card {
            count,
            brand,
            list,
            csv,
            json,
        } => {
            let registry = credit_card::Registry::new();

            if list {
                println!("Supported Brands:");
                for b in registry.list_brands() {
                    println!("  {}", b);
                }
                return;
            }

            let opts = credit_card::GenOptions {
                brand: brand.clone(),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::CARD_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match registry.generate(&opts, &mut rng) {
                    Some(r) => r,
                    None => {
                        eprintln!("Unsupported brand: {}", brand.as_deref().unwrap_or(""));
                        std::process::exit(1);
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::card_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} ({}): {}  CVV: {}  Exp: {}  (valid: {})",
                            result.brand,
                            result.formatted,
                            result.number,
                            result.cvv,
                            result.expiry,
                            result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Swift {
            count,
            country,
            csv,
            json,
        } => {
            let registry = swift::Registry::new();
            let opts = swift::GenOptions {
                country: country.clone(),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::SWIFT_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = registry.generate(&opts, &mut rng);

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::swift_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} ({}): {}  (valid: {})",
                            result.bank, result.country, result.code, result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Company {
            count,
            country,
            list,
            csv,
            json,
        } => {
            let registry = company_id::Registry::new();

            if list {
                println!("{:<6} {:<25} ID Name", "Code", "Country");
                println!("{}", "-".repeat(50));
                for (code, country_name, name) in registry.list_countries() {
                    println!("{:<6} {:<25} {}", code, country_name, name);
                }
                return;
            }

            let opts = company_id::GenOptions {
                country: country.clone(),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::COMPANY_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match registry.generate(&opts, &mut rng) {
                    Some(r) => r,
                    None => {
                        eprintln!("Unsupported country: {}", country.as_deref().unwrap_or(""));
                        std::process::exit(1);
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::company_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} - {} - {}: {}  (valid: {})",
                            result.country_code,
                            result.country_name,
                            result.name,
                            result.code,
                            result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::License {
            count,
            country,
            state,
            list,
            csv,
            json,
        } => {
            let registry = driver_license::Registry::new();

            if list {
                println!("{:<6} {:<25} ID Name", "Code", "Country");
                println!("{}", "-".repeat(50));
                for (code, country_name, name) in registry.list_countries() {
                    println!("{:<6} {:<25} {}", code, country_name, name);
                }
                return;
            }

            let opts = driver_license::GenOptions {
                country: country.clone(),
                state,
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::LICENSE_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match registry.generate(&opts, &mut rng) {
                    Some(r) => r,
                    None => {
                        eprintln!("Unsupported country: {}", country.as_deref().unwrap_or(""));
                        std::process::exit(1);
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::license_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} - {} - {}: {}  (valid: {})",
                            result.country_code,
                            result.country_name,
                            result.name,
                            result.code,
                            result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Tax {
            count,
            country,
            holder_type,
            list,
            csv,
            json,
        } => {
            let registry = tax_id::Registry::new();

            if list {
                println!("{:<6} {:<25} ID Name", "Code", "Country");
                println!("{}", "-".repeat(50));
                for (code, country_name, name) in registry.list_countries() {
                    println!("{:<6} {:<25} {}", code, country_name, name);
                }
                return;
            }

            let opts = tax_id::GenOptions {
                country: country.clone(),
                holder_type,
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::TAX_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match registry.generate(&opts, &mut rng) {
                    Some(r) => r,
                    None => {
                        eprintln!("Unsupported country: {}", country.as_deref().unwrap_or(""));
                        std::process::exit(1);
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::tax_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} - {} - {}: {}  (valid: {})",
                            result.country_code,
                            result.country_name,
                            result.name,
                            result.code,
                            result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Passport {
            count,
            country,
            list,
            csv,
            json,
        } => {
            let registry = passport::Registry::new();

            if list {
                println!("{:<6} {:<25} ID Name", "Code", "Country");
                println!("{}", "-".repeat(50));
                for (code, country_name, name) in registry.list_countries() {
                    println!("{:<6} {:<25} {}", code, country_name, name);
                }
                return;
            }

            let opts = passport::GenOptions {
                country: country.clone(),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::PASSPORT_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match registry.generate(&opts, &mut rng) {
                    Some(r) => r,
                    None => {
                        eprintln!("Unsupported country: {}", country.as_deref().unwrap_or(""));
                        std::process::exit(1);
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::passport_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} - {} - {}: {}  (valid: {})",
                            result.country_code,
                            result.country_name,
                            result.name,
                            result.code,
                            result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Vat {
            count,
            country,
            list,
            csv,
            json,
        } => {
            let registry = vat::Registry::new();

            if list {
                println!("{:<6} Country", "Code");
                println!("{}", "-".repeat(40));
                for (code, country_name) in registry.list_countries() {
                    println!("{:<6} {}", code, country_name);
                }
                return;
            }

            let opts = vat::GenOptions {
                country: country.clone(),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::VAT_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = match registry.generate(&opts, &mut rng) {
                    Some(r) => r,
                    None => {
                        eprintln!("Unsupported country: {}", country.as_deref().unwrap_or(""));
                        std::process::exit(1);
                    }
                };

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::vat_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} - {}: {}  (valid: {})",
                            result.country_code, result.country_name, result.code, result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Lei {
            count,
            country,
            csv,
            json,
        } => {
            let registry = lei::Registry::new();
            let opts = lei::GenOptions {
                country: country.clone(),
            };

            let mut out_csv: Option<Box<dyn Write>> = csv.as_deref().map(csv_writer);
            if let Some(ref mut w) = out_csv {
                writeln!(w, "{}", csv_fmt::LEI_HEADER).unwrap();
            }

            #[cfg(feature = "json")]
            let mut json_results = Vec::new();

            for _ in 0..count {
                let result = registry.generate(&opts, &mut rng);

                #[cfg(feature = "json")]
                if json.is_some() {
                    json_results.push(result.clone());
                }

                if let Some(ref mut w) = out_csv {
                    writeln!(w, "{}", csv_fmt::lei_row(&result)).unwrap();
                } else {
                    let mut print_it = true;
                    #[cfg(feature = "json")]
                    if json.is_some() {
                        print_it = false;
                    }

                    if print_it {
                        println!(
                            "{} (LOU: {}, Country: {})  (valid: {})",
                            result.code, result.lou, result.country_code, result.valid
                        );
                    }
                }
            }

            #[cfg(feature = "json")]
            if let Some(path) = json.as_deref() {
                let mut w = csv_writer(path);
                serde_json::to_writer_pretty(&mut w, &json_results).unwrap();
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }

            if let Some(path) = csv.as_deref() {
                if path != "-" {
                    eprintln!("Wrote {} rows to {}", count, path);
                }
            }
        }
        Commands::Validate {
            category,
            code,
            country,
        } => {
            let cat = category.to_lowercase();
            let country = country.map(|c| c.to_uppercase());

            let valid = match cat.as_str() {
                "iban" => iban::validate_iban(&code),
                "account" => {
                    let cc = country.unwrap_or_else(|| {
                        eprintln!("Error: --country is required for account validation");
                        std::process::exit(1);
                    });
                    bank_account::Registry::new()
                        .validate(&cc, &code)
                        .unwrap_or(false)
                }
                "id" => {
                    let cc = country.unwrap_or_else(|| {
                        eprintln!("Error: --country is required for personal ID validation");
                        std::process::exit(1);
                    });
                    personal_id::Registry::new()
                        .validate(&cc, &code)
                        .unwrap_or(false)
                }
                "card" => credit_card::Registry::new().validate(&code),
                "swift" => swift::Registry::new().validate(&code),
                "company" => {
                    let cc = country.unwrap_or_else(|| {
                        eprintln!("Error: --country is required for company ID validation");
                        std::process::exit(1);
                    });
                    company_id::Registry::new().validate(&cc, &code)
                }
                "license" => {
                    let cc = country.unwrap_or_else(|| {
                        eprintln!("Error: --country is required for driver's license validation");
                        std::process::exit(1);
                    });
                    driver_license::Registry::new().validate(&cc, &code)
                }
                "tax" => {
                    let cc = country.unwrap_or_else(|| {
                        eprintln!("Error: --country is required for tax ID validation");
                        std::process::exit(1);
                    });
                    tax_id::Registry::new().validate(&cc, &code)
                }
                "lei" => lei::Registry::new().validate(&code),
                "vat" => vat::Registry::new().validate(&code),
                "passport" => {
                    let cc = country.unwrap_or_else(|| {
                        eprintln!("Error: --country is required for passport validation");
                        std::process::exit(1);
                    });
                    passport::Registry::new().validate(&cc, &code)
                }
                _ => {
                    eprintln!(
                        "Unknown category: {}. Use iban, account, id, card, swift, company, license, tax, passport, lei, or vat.",
                        cat
                    );
                    std::process::exit(1);
                }
            };

            if valid {
                println!("TRUE: {} is a valid {} code", code, cat);
            } else {
                println!("FALSE: {} is NOT a valid {} code", code, cat);
                std::process::exit(1);
            }
        }
    }
}
