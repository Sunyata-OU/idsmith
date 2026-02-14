# Passport

Generate and validate passport numbers for 79 countries with specific format implementations.

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::passport::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::passports();

// Random country
let result = registry.generate(&GenOptions::default(), &mut rng).unwrap();

// Specific country
let opts = GenOptions { country: Some("US".to_string()) };
let us = registry.generate(&opts, &mut rng).unwrap();
// us.country_code → "US"
// us.name         → "Passport"
// us.code         → "123456789"
```

### Python
```python
import idsmith

result = idsmith.Passport.generate()                   # random country
us = idsmith.Passport.generate(country="US")           # specific country
print(us["name"])  # Passport
print(us["code"])  # 123456789
```

### JavaScript
```javascript
const { Passport } = require('idsmith');

const result = Passport.generate();      // random country
const us = Passport.generate('US');      // specific country
console.log(us.name);  // Passport
console.log(us.code);  // 123456789
```

## Validate

### Rust
```rust
idsmith::passports().validate("US", "123456789");   // true
idsmith::passports().validate("DE", "C01X00T47");   // true
```

### Python
```python
idsmith.Passport.validate("US", "123456789")  # True
```

### JavaScript
```javascript
Passport.validate('US', '123456789');  // true
```

## Country-Specific Implementations

| Country | Format | Description |
|---------|--------|-------------|
| IN | 8 chars (1 alpha + 7 digits) | Indian passport number |
| US | 9 digits | US passport number |
| GB | 9 digits | British passport number |
| DE | 9 chars (C + 8 alphanumeric) | German Reisepass with serial check digit |
| FR | 9 chars (2 alpha + 7 digits) | French passport number |
| BR | 8 chars (2 alpha + 6 digits) | Brazilian passport number |
| AU | 8 chars (1 alpha + 7 digits) | Australian passport number |
| CA | 8 chars (2 alpha + 6 digits) | Canadian passport number |
| JP | 9 chars (2 alpha + 7 digits) | Japanese passport number |
| CN | 9 chars (E/G + 8 digits) | Chinese passport (E for ordinary, G for official) |
| IT | 9 chars (2 alpha + 7 digits) | Italian passaporto |
| ES | 9 chars (3 alpha + 6 digits) | Spanish pasaporte |
| NL | 9 chars (2 alpha + 7 digits) | Dutch paspoort |
| SE | 8 digits | Swedish pass |
| KR | 9 chars (M/S + 8 digits) | Korean passport (M = regular, S = official) |
| SG | 9 chars (E + 7 digits + 1 alpha) | Singapore passport |
| ZA | 9 chars (1 alpha + 8 digits) | South African passport |
| MX | 10 digits | Mexican pasaporte |
