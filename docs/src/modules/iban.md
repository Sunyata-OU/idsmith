# IBAN

Generate and validate IBANs for 124 countries with full mod-97-10 checksum validation.

## Generate

### Rust
```rust
use rand::thread_rng;
let mut rng = thread_rng();

let iban = idsmith::iban::generate_iban(Some("DE"), &mut rng).unwrap();
let formatted = idsmith::iban::format_iban(&iban);
// DE47 5085 6216 2522 8679 09

// Random country
let random = idsmith::iban::generate_iban(None, &mut rng).unwrap();
```

### Python
```python
import idsmith

iban = idsmith.generate_iban("DE")
formatted = idsmith.format_iban(iban)

# Random country
random = idsmith.generate_iban()
```

### JavaScript
```javascript
const { generateIban, formatIban } = require('idsmith');

const iban = generateIban('DE');
const formatted = formatIban(iban);

// Random country
const random = generateIban();
```

## Validate

### Rust
```rust
idsmith::iban::validate_iban("DE47508562162522867909");  // true
```

### Python
```python
idsmith.validate_iban("DE47508562162522867909")  # True
```

### JavaScript
```javascript
validateIban('DE47508562162522867909');  // true
```

## List Supported Countries

### Rust
```rust
let countries = idsmith::iban::supported_countries();
// ["AD", "AE", "AL", "AT", ...]
```

### Python
```python
countries = idsmith.iban_countries()
# ["AD", "AE", "AL", "AT", ...]
```

### JavaScript
```javascript
const countries = ibanCountries();
// ['AD', 'AE', 'AL', 'AT', ...]
```
