# Tax ID

Generate and validate tax identification numbers for 80 countries with specific checksum and format implementations.

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::tax_id::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::tax_ids();

// Random country
let result = registry.generate(&GenOptions::default(), &mut rng).unwrap();

// Specific country
let opts = GenOptions { country: Some("IN".to_string()), holder_type: None };
let india = registry.generate(&opts, &mut rng).unwrap();
// india.country_code → "IN"
// india.name         → "PAN"
// india.code         → "ABCDE1234F"

// India PAN with holder type
let opts = GenOptions {
    country: Some("IN".to_string()),
    holder_type: Some("C".to_string()),  // C = Company
};
let pan = registry.generate(&opts, &mut rng).unwrap();
```

### Python
```python
import idsmith

result = idsmith.TaxId.generate()                          # random country
india = idsmith.TaxId.generate(country="IN")               # specific country
print(india["name"])  # PAN
print(india["code"])  # ABCDE1234F

# India PAN with holder type
pan = idsmith.TaxId.generate(country="IN", holder_type="C")
```

### JavaScript
```javascript
const { TaxId } = require('idsmith');

const result = TaxId.generate();           // random country
const india = TaxId.generate('IN');        // specific country
console.log(india.name);  // PAN
console.log(india.code);  // ABCDE1234F

// India PAN with holder type
const pan = TaxId.generate('IN', { holderType: 'C' });
```

## Validate

### Rust
```rust
idsmith::tax_ids().validate("IN", "ABCDE1234F");   // true
idsmith::tax_ids().validate("BR", "12345678909");   // true
```

### Python
```python
idsmith.TaxId.validate("IN", "ABCDE1234F")  # True
```

### JavaScript
```javascript
TaxId.validate('IN', 'ABCDE1234F');  // true
```

## Checksum-Verified Countries

| Country | Format | Checksum |
|---------|--------|----------|
| IN | PAN (10 chars, AAAAA0000A) | Pattern validation, holder_type option (P/C/H/F/A/T/B/L/J/G) |
| US | TIN (9 digits) | Area number constraints (no 000, 666, 9xx) |
| GB | UTR (10 digits) | Weighted mod-11 checksum |
| DE | Steuer-IdNr (11 digits) | ISO 7064 mod 11,10 |
| FR | NIF (13 digits) | Mod-97 checksum |
| BR | CPF (11 digits) | Mod-11 checksum (two check digits) |
| AU | TFN (9 digits) | Weighted checksum (mod 11) |
| CA | SIN (9 digits) | Luhn checksum |
| JP | My Number (12 digits) | Mod-11 weighted checksum |
| CN | USCI (18 chars alphanumeric) | Mod-31 weighted checksum |
| IT | Partita IVA (11 digits) | Luhn-variant checksum |
| ES | NIF (8 digits + letter) | Mod-23 check letter |
| NL | BSN (9 digits) | 11-check (weighted sum) |
| SE | Personnummer (10 digits) | Luhn checksum |
| KR | BRN (10 digits) | Weighted check digit |
| SG | Tax Ref (9 chars, NRIC format) | Weighted mod-11 with check letter |
| ZA | Tax Number (10 digits) | Luhn checksum |
| MX | RFC (12-13 chars) | Mod-11 alphanumeric check digit |
