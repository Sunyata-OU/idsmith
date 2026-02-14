# Node.js API Reference

## Classes

### `BankAccount`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: string, bankCode?: string)` | `AccountResult` |
| `validate` | `(country: string, raw: string)` | `boolean` |
| `format` | `(country: string, raw: string)` | `string` |
| `listCountries` | `()` | `BankCountryInfo[]` |
| `isSupported` | `(country: string)` | `boolean` |

```typescript
interface AccountResult {
  countryCode: string;
  countryName: string;
  formatName: string;
  bankCode: string | null;
  branchCode: string | null;
  accountNumber: string;
  checkDigits: string | null;
  formatted: string;
  raw: string;
  iban: string | null;
  valid: boolean;
}

interface BankCountryInfo {
  code: string;
  name: string;
  format: string;
  hasIban: boolean;
}
```

---

### `PersonalId`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: string, gender?: string, year?: number)` | `string` |
| `validate` | `(country: string, code: string)` | `boolean` |
| `parse` | `(country: string, code: string)` | `IdResult` |
| `listCountries` | `()` | `CountryInfo[]` |
| `isSupported` | `(country: string)` | `boolean` |

**`gender` parameter:** `"male"` or `"female"` (or omit for random).

```typescript
interface IdResult {
  countryCode: string;
  code: string;
  gender: string | null;
  dob: string | null;
  valid: boolean;
}
```

---

### `CreditCard`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(brand?: string)` | `CardResult` |
| `validate` | `(number: string)` | `boolean` |
| `format` | `(brand: string, number: string)` | `string` |
| `listBrands` | `()` | `string[]` |

**Supported brands:** `visa`, `mastercard`, `amex`, `discover`, `jcb`, `diners`

```typescript
interface CardResult {
  brand: string;
  number: string;
  formatted: string;
  cvv: string;
  expiry: string;
  valid: boolean;
}
```

---

### `CompanyId`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country?: string)` | `CompanyResult` |
| `validate` | `(country: string, code: string)` | `boolean` |
| `listCountries` | `()` | `CountryInfo[]` |

```typescript
interface CompanyResult {
  countryCode: string;
  countryName: string;
  name: string;
  code: string;
  valid: boolean;
}
```

---

### `Swift`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country?: string)` | `SwiftResult` |
| `validate` | `(code: string)` | `boolean` |

```typescript
interface SwiftResult {
  code: string;
  bank: string;
  country: string;
  location: string;
  branch: string | null;
  valid: boolean;
}
```

---

### `DriverLicense`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country?: string)` | `DriverLicenseResult` |
| `validate` | `(country: string, code: string)` | `boolean` |
| `listCountries` | `()` | `CountryInfo[]` |
| `isSupported` | `(country: string)` | `boolean` |

```typescript
interface DriverLicenseResult {
  countryCode: string;
  countryName: string;
  name: string;
  code: string;
  valid: boolean;
}
```

---

### `TaxId`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country?: string, options?: TaxIdOptions)` | `TaxIdResult` |
| `validate` | `(country: string, code: string)` | `boolean` |
| `listCountries` | `()` | `CountryInfo[]` |
| `isSupported` | `(country: string)` | `boolean` |

```typescript
interface TaxIdOptions {
  holderType?: string;  // IN only: P, C, H, F, A, T, B, L, J, G
}

interface TaxIdResult {
  countryCode: string;
  countryName: string;
  name: string;
  code: string;
  valid: boolean;
}
```

---

### `Passport`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country?: string)` | `PassportResult` |
| `validate` | `(country: string, code: string)` | `boolean` |
| `listCountries` | `()` | `CountryInfo[]` |
| `isSupported` | `(country: string)` | `boolean` |

```typescript
interface PassportResult {
  countryCode: string;
  countryName: string;
  name: string;
  code: string;
  valid: boolean;
}
```

---

## Functions

| Function | Signature | Returns |
|----------|-----------|---------|
| `generateIban` | `(country?: string)` | `string` |
| `validateIban` | `(iban: string)` | `boolean` |
| `formatIban` | `(iban: string)` | `string` |
| `ibanCountries` | `()` | `string[]` |

---

## Shared Types

```typescript
interface CountryInfo {
  code: string;
  countryName: string;
  idName: string;
}
```
