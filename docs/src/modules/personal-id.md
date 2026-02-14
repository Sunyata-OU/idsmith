# Personal ID

Generate, validate, and parse national identity numbers for 97 countries with checksum verification (SSN, CPF, Aadhaar, PESEL, etc.).

## Generate

### Rust
```rust
use rand::thread_rng;
use idsmith::personal_id::{GenOptions, date::Gender};

let mut rng = thread_rng();
let registry = idsmith::personal_ids();

// Default options (random gender, random year)
let id = registry.generate("EE", &Default::default(), &mut rng).unwrap();

// With gender and year
let opts = GenOptions {
    gender: Some(Gender::Female),
    year: Some(1990),
};
let id = registry.generate("EE", &opts, &mut rng).unwrap();
```

### Python
```python
import idsmith

# Default options
code = idsmith.PersonalId.generate("EE")

# With gender and year
code = idsmith.PersonalId.generate("EE", gender="female", year=1990)
```

### JavaScript
```javascript
const { PersonalId } = require('idsmith');

// Default options
const code = PersonalId.generate('EE');

// With gender and year
const code2 = PersonalId.generate('EE', 'female', 1990);
```

## Validate

### Rust
```rust
let valid = idsmith::personal_ids().validate("US", "446-72-2445").unwrap();
```

### Python
```python
valid = idsmith.PersonalId.validate("US", "446-72-2445")  # True
```

### JavaScript
```javascript
const valid = PersonalId.validate('US', '446-72-2445');  // true
```

## Parse

Extract metadata (date of birth, gender) from an ID.

### Rust
```rust
let result = idsmith::personal_ids().parse("EE", "49001011234").unwrap();
// result.dob    → Some("1990-01-01")
// result.gender → Some("female")
// result.valid  → true
```

### Python
```python
result = idsmith.PersonalId.parse("EE", "49001011234")
# result["dob"]    → "1990-01-01"
# result["gender"] → "female"
# result["valid"]  → True
```

### JavaScript
```javascript
const result = PersonalId.parse('EE', '49001011234');
// result.dob    → '1990-01-01'
// result.gender → 'female'
// result.valid  → true
```

## Checksum-Verified Countries (56)

| Region | Countries |
|--------|-----------|
| Europe | EE, LT, LV, FI, SE, NO, DK, IS, DE, AT, CH, NL, BE, FR, ES, PT, IT, GB, IE, PL, CZ, SK, RO, BG, HR, SI, RS, BA, ME, TR, GR |
| Americas | US, CA, BR, AR, CL, CO, UY, EC, PE, MX |
| Asia-Pacific | CN, IN, JP, KR, TW, TH, SG, MY, ID, HK, AU, NZ |
| Africa/Middle East | ZA, IL, EG |
