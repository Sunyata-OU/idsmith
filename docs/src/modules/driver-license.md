# Driver's License

Generate and validate driver's license numbers for 79 countries with specific checksum and format implementations.

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::driver_license::GenOptions;

let mut rng = thread_rng();
let registry = idsmith::driver_licenses();

// Random country
let result = registry.generate(&GenOptions::default(), &mut rng).unwrap();

// Specific country
let opts = GenOptions { country: Some("US".to_string()), state: None };
let us = registry.generate(&opts, &mut rng).unwrap();
// us.country_code → "US"
// us.name         → "Driver's License"
// us.code         → "A123456789012"
```

### Python
```python
import idsmith

result = idsmith.DriverLicense.generate()                   # random country
us = idsmith.DriverLicense.generate(country="US")           # specific country
print(us["name"])  # Driver's License
print(us["code"])  # A123456789012
```

### JavaScript
```javascript
const { DriverLicense } = require('idsmith');

const result = DriverLicense.generate();      // random country
const us = DriverLicense.generate('US');      // specific country
console.log(us.name);  // Driver's License
console.log(us.code);  // A123456789012
```

## Validate

### Rust
```rust
idsmith::driver_licenses().validate("US", "A123456789012");  // true
idsmith::driver_licenses().validate("IN", "KA0120190012345"); // true
```

### Python
```python
idsmith.DriverLicense.validate("US", "A123456789012")  # True
```

### JavaScript
```javascript
DriverLicense.validate('US', 'A123456789012');  // true
```

## Country-Specific Implementations

| Country | Format | Description |
|---------|--------|-------------|
| IN | 15 chars ({STATE}{RTO}{YEAR}{SERIAL}) | Indian DL with state, RTO code, year, and serial |
| US | 13 chars (1 alpha + 12 digits) | US driver's license number |
| GB | 16 chars DVLA alphanumeric | DVLA format with name-derived characters |
| DE | 11 chars alphanumeric | German Führerschein number |
| FR | 12 chars (2 alpha + 10 digits) | French permis de conduire |
| BR | 11 digits with 2 check digits (CNH) | Brazilian CNH with mod-based check digits |
| AU | 8-10 chars alphanumeric | Australian state-issued license |
| CA | 13 chars (1 alpha + 12 digits) | Canadian provincial license |
| JP | 12 digits | Japanese driver's license number |
| CN | 12 digits | Chinese driver's license number |
| IT | 10 chars (2 alpha + 7 digits + 1 alpha) | Italian patente di guida |
| ES | 9 chars (8 digits + check letter) | Spanish DNI-format with mod-23 check |
| NL | 10 digits | Dutch rijbewijs number |
| SE | 10 digits (personnummer format) | Swedish körkort with Luhn checksum |
| KR | 12 digits (region + year + serial) | Korean driver's license with region codes |
| SG | 9 chars (NRIC format) | Singapore license with weighted checksum |
| ZA | 13 digits (ID number format) | South African license with Luhn checksum |
| MX | 12 chars (CURP-derived) | Mexican licencia de conducir |
