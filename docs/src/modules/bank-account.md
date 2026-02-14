# Bank Account

Generate, validate, and format bank account numbers for 159 countries. Includes specific implementations for 16 major economies with IBAN support for 124 countries.

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::bank_account::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::bank_accounts();

let result = registry.generate("US", &GenOptions::default(), &mut rng).unwrap();
// result.formatted → "021000021 | 123456789"
// result.raw       → "021000021123456789"
// result.iban      → None (US doesn't use IBAN)

// IBAN countries include the IBAN
let de = registry.generate("DE", &GenOptions::default(), &mut rng).unwrap();
// de.iban → Some("DE47508562162522867909")
```

### Python
```python
import idsmith

result = idsmith.BankAccount.generate("US")
print(result["formatted"])  # 021000021 | 123456789
print(result["iban"])       # None

# With bank code filter
result = idsmith.BankAccount.generate("US", bank_code="021000021")
```

### JavaScript
```javascript
const { BankAccount } = require('idsmith');

const result = BankAccount.generate('US');
console.log(result.formatted);  // 021000021 | 123456789
console.log(result.iban);       // null

// With bank code filter
const result2 = BankAccount.generate('US', '021000021');
```

## Validate

### Rust
```rust
let valid = idsmith::bank_accounts().validate("MX", "167078019952865929").unwrap();
```

### Python
```python
valid = idsmith.BankAccount.validate("MX", "167078019952865929")
```

### JavaScript
```javascript
const valid = BankAccount.validate('MX', '167078019952865929');
```

## Format

### Rust
```rust
let formatted = idsmith::bank_accounts().format("US", "021000021123456789").unwrap();
```

### Python
```python
formatted = idsmith.BankAccount.format("US", "021000021123456789")
```

### JavaScript
```javascript
const formatted = BankAccount.format('US', '021000021123456789');
```

## Specific Country Formats

| Country | Format | Has IBAN |
|---------|--------|----------|
| US | ABA Routing + Account | No |
| CA | Institution + Transit + Account | No |
| MX | CLABE (18 digits) | No |
| AU | BSB + Account | No |
| IN | IFSC + Account | No |
| JP | Bank + Branch + Account | No |
| CN | Bank Account (Luhn) | No |
| GB | Sort Code + Account | Yes |
| BR | Bank + Branch + Account | No |
| AR | CBU | No |
| NG | NUBAN | No |
| + 85 | IBAN-based accounts | Yes |
| + 140 | Generic bank formats | Varies |
