<p align="center">
  <img src="logo.png" alt="idsmith logo" width="200" />
</p>

# idsmith

[![Crates.io](https://img.shields.io/crates/v/idsmith)](https://crates.io/crates/idsmith)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://sunyata-ou.github.io/idsmith/)
[![CI](https://github.com/Sunyata-OU/idsmith/actions/workflows/ci.yml/badge.svg)](https://github.com/Sunyata-OU/idsmith/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Validate and generate checksum-correct **IBANs**, **personal IDs**, **bank accounts**, **credit cards**, **SWIFT/BIC**, **company IDs**, **driver's licenses**, **tax IDs**, **passports**, **LEI codes**, and **EU VAT numbers**.

Available as a **Rust crate**, **Python package**, and **Node.js module** — all powered by the same Rust core.

**[Read the full documentation →](https://sunyata-ou.github.io/idsmith/)**

## Install

```bash
# Rust (library only, no CLI deps)
cargo add idsmith --no-default-features

# Rust (with JSON serialization)
cargo add idsmith --no-default-features --features json

# Rust (CLI)
cargo install idsmith

# Python
pip install idsmith

# Node.js
npm install idsmith
```

### Cargo Features

| Feature | Description | Default |
|---------|-------------|---------|
| `cli` | Full CLI binary (clap, csv, json) | Yes |
| `json` | `serde::Serialize` on all result types | No |
| `csv` | CSV output formatting | No |

Use `default-features = false` when using as a library to keep dependencies minimal.

## Quick Example

```rust
// Rust
use idsmith::{credit_cards, personal_ids, tax_ids, passports, driver_licenses, vat_ids, lei_codes};

let valid = credit_cards().validate("4152839405126374");
let ssn_ok = personal_ids().validate("US", "446-72-2445").unwrap_or(false);
let pan_ok = tax_ids().validate("IN", "ABCDE1234F");
let passport_ok = passports().validate("US", "123456789");
let dl_ok = driver_licenses().validate("US", "A123456789012");
let vat_ok = vat_ids().validate("DE123456789");
let lei_ok = lei_codes().validate("529900BOTDR0SE98AR17");
```

```python
# Python
import idsmith

idsmith.CreditCard.validate("4152839405126374")      # True
idsmith.PersonalId.validate("US", "446-72-2445")      # True
idsmith.TaxId.validate("IN", "ABCDE1234F")            # True
idsmith.Passport.generate(country="DE")
idsmith.DriverLicense.validate("US", "A123456789012")  # True
idsmith.VatId.validate("DE123456789")                  # True
idsmith.LegalEntityId.validate("529900BOTDR0SE98AR17") # True
iban = idsmith.generate_iban("DE")
```

```javascript
// Node.js
const { CreditCard, PersonalId, TaxId, Passport, DriverLicense, VatId, LegalEntityId, generateIban } = require('idsmith');

CreditCard.validate('4152839405126374');      // true
PersonalId.validate('US', '446-72-2445');     // true
TaxId.validate('IN', 'ABCDE1234F');           // true
const passport = Passport.generate('DE');
DriverLicense.validate('US', 'A123456789012'); // true
VatId.validate('DE123456789');                // true
LegalEntityId.validate('529900BOTDR0SE98AR17'); // true
const iban = generateIban('DE');
```

## Features

- **124 IBAN countries** with mod-97-10 checksum validation
- **159 bank account formats** — US ABA, MX CLABE, AU BSB, IN IFSC, and more
- **97 personal ID formats** — SSN, CPF, Aadhaar, PESEL, Codice Fiscale, etc.
- **6 credit card brands** — Visa, Mastercard, Amex, Discover, JCB, Diners (Luhn)
- **SWIFT/BIC codes** — valid 8 and 11 character codes
- **250 company ID formats** — VAT numbers, EINs, CIFs with checksums
- **79 driver's license formats** — with country-specific checksum and format validation
- **80 tax ID formats** — with checksum validation (PAN, TIN, CPF, SIN, Steuer-IdNr, USCI, Partita IVA, NIF, BSN, RFC, and more)
- **79 passport formats** — with country-specific format validation
- **LEI codes** — ISO 17442 Legal Entity Identifiers with mod-97 checksum
- **28 EU VAT number formats** — all EU member states + GB with country-specific checksums (cross-validated against python-stdnum)
- **CLI tool** with JSON and CSV export

## Performance

IBAN validation throughput (100k iterations, single-threaded):

| Library | Language | Throughput | vs idsmith |
|---------|----------|-----------|------------|
| **idsmith** | **Rust** | **~1,310,000 ops/s** | **—** |
| `ibantools` | Node.js | ~460,000 ops/s | ~2.8x slower |
| `python-stdnum` | Python | ~54,000 ops/s | ~24x slower |

Extended document validation (idsmith Rust):

| Document Type | Throughput | vs Node.js alternatives |
|---------------|------------|-------------------------|
| Personal ID (US SSN) | ~9,300,000 ops/s | ~30x faster |
| Credit Card (Visa) | ~14,900,000 ops/s | ~53x faster |
| Driver License (US) | ~10,100,000 ops/s | — |
| Passport (DE) | ~19,100,000 ops/s | — |
| Tax ID (India PAN) | ~7,800,000 ops/s | — |

Python and Node.js bindings call the same Rust core — same speed, same correctness.

## Projects Using idsmith

- [MockBanker](https://tonybenoy.github.io/mockbanker/) — Web app implementing nearly all idsmith features, generate realistic mock banking and identity data in the browser

## Documentation

- [Full Documentation](https://sunyata-ou.github.io/idsmith/)
- [Rust API Reference](https://sunyata-ou.github.io/idsmith/api/rust/idsmith/index.html)
- [Python Quick Start](https://sunyata-ou.github.io/idsmith/python/quick-start.html)
- [Node.js Quick Start](https://sunyata-ou.github.io/idsmith/node/quick-start.html)
- [CLI Usage](https://sunyata-ou.github.io/idsmith/rust/cli.html)

## License

MIT
