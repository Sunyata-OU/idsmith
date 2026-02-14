<p align="center">
  <img src="logo.png" alt="idsmith logo" width="200" />
</p>

# idsmith

[![Crates.io](https://img.shields.io/crates/v/idsmith)](https://crates.io/crates/idsmith)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://sunyata-ou.github.io/idsmith/)
[![CI](https://github.com/Sunyata-OU/idsmith/actions/workflows/ci.yml/badge.svg)](https://github.com/Sunyata-OU/idsmith/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Validate and generate checksum-correct **IBANs**, **personal IDs**, **bank accounts**, **credit cards**, **SWIFT/BIC**, and **company IDs** for 252 countries.

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
use idsmith::{credit_cards, personal_ids};

let valid = credit_cards().validate("4152839405126374");
let ssn_ok = personal_ids().validate("US", "446-72-2445").unwrap_or(false);
```

```python
# Python
import idsmith

idsmith.CreditCard.validate("4152839405126374")      # True
idsmith.PersonalId.validate("US", "446-72-2445")      # True
iban = idsmith.generate_iban("DE")
```

```javascript
// Node.js
const { CreditCard, PersonalId, generateIban } = require('idsmith');

CreditCard.validate('4152839405126374');      // true
PersonalId.validate('US', '446-72-2445');     // true
const iban = generateIban('DE');
```

## Features

- **96 IBAN countries** with mod-97-10 checksum validation
- **252 bank account formats** — US ABA, MX CLABE, AU BSB, IN IFSC, and more
- **56 checksum-verified personal IDs** — SSN, CPF, Aadhaar, PESEL, Codice Fiscale, etc.
- **6 credit card brands** — Visa, Mastercard, Amex, Discover, JCB, Diners (Luhn)
- **SWIFT/BIC codes** — valid 8 and 11 character codes
- **252 company ID formats** — VAT numbers, EINs, CIFs with checksums
- **CLI tool** with JSON and CSV export

## Performance

IBAN validation throughput (100k iterations, single-threaded):

| Library | Language | Throughput | vs idsmith |
|---------|----------|-----------|------------|
| **idsmith** | **Rust** | **~1,350,000 ops/s** | **—** |
| `ibantools` | Node.js | ~490,000 ops/s | ~2.7x slower |
| `python-stdnum` | Python | ~53,000 ops/s | ~25x slower |

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
