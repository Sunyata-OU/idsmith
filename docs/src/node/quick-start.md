# Node.js Quick Start

## Installation

```bash
npm install idsmith
```

## Validation

```javascript
const {
  BankAccount, PersonalId, CreditCard, CompanyId,
  Swift, DriverLicense, Passport, TaxId, validateIban
} = require('idsmith');

// IBAN
validateIban('DE47508562162522867909');  // true

// Personal ID
PersonalId.validate('US', '446-72-2445');  // true

// Credit Card
CreditCard.validate('4152839405126374');  // true

// Bank Account
BankAccount.validate('US', '021000021-123456789');  // true

// Company ID
CompanyId.validate('GB', 'GB123456789');  // true

// Driver's License
DriverLicense.validate('US', 'A123456789012');  // true

// Passport
Passport.validate('DE', 'C01234567');  // true

// Tax ID (TIN)
TaxId.validate('IN', 'ABCDE1234F');  // true

// SWIFT/BIC
Swift.validate('CHASGB2LXXX');  // true
```

## Generation

```javascript
const {
  BankAccount, PersonalId, CreditCard, CompanyId, Swift,
  DriverLicense, Passport, TaxId,
  generateIban, formatIban
} = require('idsmith');

// Generate an IBAN
const iban = generateIban('DE');
console.log(formatIban(iban));  // DE47 5085 6216 2522 8679 09

// Generate a bank account
const account = BankAccount.generate('US');
console.log(account.formatted);    // 021000021 | 123456789
console.log(account.countryCode);  // US

// Generate a personal ID with options
const code = PersonalId.generate('EE', 'female', 1990);
console.log(code);  // 49001011234

// Generate a credit card
const card = CreditCard.generate('visa');
console.log(card.formatted);  // 4152 8394 0512 6374
console.log(card.brand);      // VISA

// Generate a company ID
const company = CompanyId.generate('GB');
console.log(company.code);  // GB123456789

// Generate a SWIFT code
const swift = Swift.generate('US');
console.log(swift.code);  // CHASUSU5XXX

// Generate a Driver's License
const dl = DriverLicense.generate('US');

// Generate a Passport
const passport = Passport.generate('DE');

// Generate a Tax ID (TIN)
const tin = TaxId.generate('IN');
```

## TypeScript

Full TypeScript support is included. Types are auto-generated from Rust definitions.

```typescript
import {
  BankAccount, PersonalId, CreditCard,
  generateIban, validateIban,
  type AccountResult, type CardResult
} from 'idsmith';

const account: AccountResult = BankAccount.generate('US');
const card: CardResult = CreditCard.generate('visa');
const valid: boolean = validateIban('DE47508562162522867909');
```

## Parsing

```javascript
const result = PersonalId.parse('EE', '49001011234');
console.log(result.dob);     // 1990-01-01
console.log(result.gender);  // female
console.log(result.valid);   // true
```

## Listing Supported Countries

```javascript
const { BankAccount, PersonalId, CreditCard, ibanCountries } = require('idsmith');

// IBAN countries
const countries = ibanCountries();  // ['AD', 'AE', 'AL', ...]

// Bank account countries
BankAccount.listCountries().forEach(c => {
  console.log(`${c.code} - ${c.name} (${c.format})`);
});

// Personal ID countries
PersonalId.listCountries().forEach(c => {
  console.log(`${c.code} - ${c.countryName} (${c.idName})`);
});

// Credit card brands
CreditCard.listBrands();  // ['Visa', 'Mastercard', ...]

// Check support
BankAccount.isSupported('US');  // true
PersonalId.isSupported('EE');   // true
```

## Error Handling

Unsupported countries throw an error:

```javascript
try {
  PersonalId.generate('XX');
} catch (e) {
  console.error(e.message);  // Unsupported country: XX
}
```
