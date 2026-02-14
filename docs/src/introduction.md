# idsmith

Validate and generate checksum-correct **IBANs**, **personal IDs**, **bank accounts**, **credit cards**, **SWIFT/BIC**, **company IDs**, **driver's licenses**, **tax IDs**, and **passports** across multiple countries.

Built for developers and QA engineers who need a robust way to validate existing identifiers or create realistic, algorithmically valid test data.

## Features

- **Validator + Generator** — verify existing strings or create new data
- **124 IBAN countries** — full IBAN registry coverage with mod-97-10 checksum validation
- **159 bank account formats** — US ABA routing, MX CLABE, AU BSB, IN IFSC, AR CBU, NG NUBAN, BR mod-11, etc.
- **97 personal ID formats** — checksum-verified (SSN, CPF, Aadhaar, Resident ID, and more)
- **250 company ID formats** — VAT numbers, EINs, and Business IDs with major economy checksums
- **Universal Credit Cards** — Visa, Mastercard, Amex, Discover, JCB, and Diners Club with Luhn support
- **SWIFT/BIC** — valid 8 and 11 character codes with ISO country positioning
- **79 driver's license formats** — with country-specific checksum and format validation
- **80 tax ID formats** — with checksum validation (PAN, TIN, CPF, SIN, Steuer-IdNr, USCI, Partita IVA, NIF, BSN, RFC, and more)
- **79 passport formats** — with country-specific format validation

## Available in 3 Languages

| Platform | Package | Install |
|----------|---------|---------|
| **Rust** | [idsmith](https://crates.io/crates/idsmith) | `cargo add idsmith` |
| **Python** | [idsmith](https://pypi.org/project/idsmith/) | `pip install idsmith` |
| **Node.js** | [idsmith](https://www.npmjs.com/package/idsmith) | `npm install idsmith` |

The Python and Node.js packages are native bindings to the Rust core — same speed, same correctness, idiomatic APIs.

## Performance

| Library | Language | Throughput | Relative Speed |
| :--- | :--- | :--- | :--- |
| **idsmith** | **Rust** | **~1,280,000 ops/s** | **1.0x** |
| `ibantools` | Node.js | ~460,000 ops/s | ~2.7x slower |
| `python-stdnum`| Python | ~54,000 ops/s | ~23x slower |

## Validation Standards

| Category | Validation Source |
|----------|-------------------|
| IBAN | [ibantools](https://www.npmjs.com/package/ibantools) |
| Personal ID | [python-stdnum](https://github.com/arthurdejong/python-stdnum), [taiwan-id](https://www.npmjs.com/package/taiwan-id) |
| Bank Account | [abavalidator](https://www.npmjs.com/package/abavalidator), [clabe-validator](https://github.com/center-key/clabe-validator) |
| Credit Card | ISO/IEC 7812 (Luhn Algorithm) |
| SWIFT/BIC | ISO 9362 Standards |
