# Company ID

Generate and validate business identifiers (VAT numbers, EINs, CIFs) for 250 countries. Specific checksum implementations for GB, DE, FR, IT, and ES.

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::company_id::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::company_ids();

// Random country
let result = registry.generate(&GenOptions::default(), &mut rng).unwrap();

// Specific country
let opts = GenOptions { country: Some("GB".to_string()) };
let gb = registry.generate(&opts, &mut rng).unwrap();
// gb.country_code → "GB"
// gb.name         → "VAT Number"
// gb.code         → "GB123456789"
```

### Python
```python
import idsmith

result = idsmith.CompanyId.generate()                   # random country
gb = idsmith.CompanyId.generate(country="GB")           # specific country
print(gb["name"])  # VAT Number
print(gb["code"])  # GB123456789
```

### JavaScript
```javascript
const { CompanyId } = require('idsmith');

const result = CompanyId.generate();      // random country
const gb = CompanyId.generate('GB');      // specific country
console.log(gb.name);  // VAT Number
console.log(gb.code);  // GB123456789
```

## Validate

### Rust
```rust
idsmith::company_ids().validate("GB", "GB123456789");  // true
idsmith::company_ids().validate("DE", "DE141158922");  // true
```

### Python
```python
idsmith.CompanyId.validate("GB", "GB123456789")  # True
```

### JavaScript
```javascript
CompanyId.validate('GB', 'GB123456789');  // true
```

## Checksum-Verified Countries

| Country | Format | Checksum |
|---------|--------|----------|
| GB | VAT Number (9 digits) | Weighted mod-97 |
| DE | USt-IdNr (9 digits) | ISO 7064 mod 11,10 |
| FR | TVA (11 digits) | SIREN + mod-97 |
| IT | Partita IVA (11 digits) | Luhn |
| ES | CIF (letter + 7 digits + check) | Custom weighted |
