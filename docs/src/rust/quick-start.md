# Rust Quick Start

## Setup

```toml
[dependencies]
idsmith = { version = "0.4.0", default-features = false }
rand = "0.8"
```

## Validation

```rust
use idsmith::{credit_cards, personal_ids, bank_accounts, company_ids, swift_codes};

// Simple validation (returns bool)
let card_ok = credit_cards().validate("4152839405126374");
let swift_ok = swift_codes().validate("PBIHNLY9XXX");

// Country-specific validation (returns Option<bool>)
let ssn_ok = personal_ids().validate("US", "446-72-2445").unwrap_or(false);
let vat_ok = company_ids().validate("DE", "DE141158922");
let dl_ok = idsmith::driver_licenses().validate("US", "A123456789012");
let passport_ok = idsmith::passports().validate("DE", "C01234567");
let tin_ok = idsmith::tax_ids().validate("IN", "ABCDE1234F");
let acc_ok = bank_accounts().validate("MX", "167078019952865929").unwrap_or(false);

// IBAN validation
let iban_ok = idsmith::iban::validate_iban("DE47508562162522867909");
```

## Generation

```rust
use rand::thread_rng;
use idsmith::{credit_cards, personal_ids, bank_accounts};

let mut rng = thread_rng();

// Generate with default options
let card = credit_cards().generate(&Default::default(), &mut rng).unwrap();
println!("{} - {}", card.brand, card.formatted);

// Generate a US bank account
let opts = idsmith::bank_account::GenOptions { bank_code: None };
let account = bank_accounts().generate("US", &opts, &mut rng).unwrap();
println!("{}", account.formatted);

// Generate a Brazilian personal ID (CPF)
let id = personal_ids().generate("BR", &Default::default(), &mut rng).unwrap();
println!("{}", id);

// Generate a German IBAN
let iban = idsmith::iban::generate_iban(Some("DE"), &mut rng).unwrap();
println!("{}", idsmith::iban::format_iban(&iban));

// Generate a Passport
let passport = idsmith::passports().generate(&Default::default(), &mut rng).unwrap();

// Generate a Tax ID (TIN)
let tin = idsmith::tax_ids().generate(&Default::default(), &mut rng).unwrap();
```

## Parsing

```rust
let result = personal_ids().parse("FI", "050497-598S").unwrap();
println!("DOB: {:?}, Gender: {:?}", result.dob, result.gender);
```

## Optional Features

| Feature | Description |
|---------|-------------|
| `json` | Enables `serde::Serialize` on all result types |
| `csv` | Enables CSV output formatting |
| `cli` | Full CLI binary (enabled by default) |

```toml
# Library only — minimal dependencies
idsmith = { version = "0.4.0", default-features = false }

# Library with JSON serialization
idsmith = { version = "0.4.0", default-features = false, features = ["json"] }
```
