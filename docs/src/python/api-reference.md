# Python API Reference

## Module: `idsmith`

### Classes

#### `BankAccount`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str, bank_code: str \| None = None)` | `dict` |
| `validate` | `(country: str, raw: str)` | `bool` |
| `format` | `(country: str, raw: str)` | `str` |
| `list_countries` | `()` | `list[dict]` |
| `is_supported` | `(country: str)` | `bool` |

**`generate` returns:**
```python
{
    "country_code": "US",
    "country_name": "United States",
    "format_name": "ABA Routing + Account",
    "bank_code": "021000021",       # or None
    "branch_code": None,             # or str
    "account_number": "123456789",
    "check_digits": None,            # or str
    "formatted": "021000021 | 123456789",
    "raw": "021000021123456789",
    "iban": None,                    # or str (for IBAN countries)
    "valid": True
}
```

---

#### `PersonalId`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str, gender: str \| None = None, year: int \| None = None)` | `str` |
| `validate` | `(country: str, code: str)` | `bool` |
| `parse` | `(country: str, code: str)` | `dict` |
| `list_countries` | `()` | `list[dict]` |
| `is_supported` | `(country: str)` | `bool` |

**`gender` parameter:** `"male"` or `"female"` (or `None` for random).

**`parse` returns:**
```python
{
    "country_code": "EE",
    "code": "49001011234",
    "gender": "female",   # or None
    "dob": "1990-01-01",  # or None
    "valid": True
}
```

---

#### `CreditCard`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(brand: str \| None = None)` | `dict` |
| `validate` | `(number: str)` | `bool` |
| `format` | `(brand: str, number: str)` | `str` |
| `list_brands` | `()` | `list[str]` |

**Supported brands:** `visa`, `mastercard`, `amex`, `discover`, `jcb`, `diners`

**`generate` returns:**
```python
{
    "brand": "VISA",
    "number": "4152839405126374",
    "formatted": "4152 8394 0512 6374",
    "cvv": "123",
    "expiry": "09/28",
    "valid": True
}
```

---

#### `CompanyId`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str \| None = None)` | `dict` |
| `validate` | `(country: str, code: str)` | `bool` |
| `list_countries` | `()` | `list[dict]` |

**`generate` returns:**
```python
{
    "country_code": "GB",
    "country_name": "United Kingdom",
    "name": "VAT Number",
    "code": "GB123456789",
    "valid": True
}
```

---

#### `Swift`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str \| None = None)` | `dict` |
| `validate` | `(code: str)` | `bool` |

**`generate` returns:**
```python
{
    "code": "CHASUSU5XXX",
    "bank": "CHAS",
    "country": "US",
    "location": "U5",
    "branch": "XXX",   # or None
    "valid": True
}
```

---

#### `DriverLicense`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str \| None = None)` | `dict` |
| `validate` | `(country: str, code: str)` | `bool` |
| `list_countries` | `()` | `list[dict]` |
| `is_supported` | `(country: str)` | `bool` |

**`generate` returns:**
```python
{
    "country_code": "US",
    "country_name": "United States",
    "name": "Driver's License",
    "code": "A123456789012",
    "valid": True
}
```

---

#### `TaxId`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str \| None = None, holder_type: str \| None = None)` | `dict` |
| `validate` | `(country: str, code: str)` | `bool` |
| `list_countries` | `()` | `list[dict]` |
| `is_supported` | `(country: str)` | `bool` |

**`holder_type` parameter (IN only):** `"P"` (Person), `"C"` (Company), `"H"` (HUF), `"F"` (Firm), `"A"` (AOP), `"T"` (Trust), `"B"` (BOI), `"L"` (Local Authority), `"J"` (Artificial Juridical Person), `"G"` (Government).

**`generate` returns:**
```python
{
    "country_code": "IN",
    "country_name": "India",
    "name": "PAN",
    "code": "ABCDE1234F",
    "valid": True
}
```

---

#### `Passport`

| Method | Signature | Returns |
|--------|-----------|---------|
| `generate` | `(country: str \| None = None)` | `dict` |
| `validate` | `(country: str, code: str)` | `bool` |
| `list_countries` | `()` | `list[dict]` |
| `is_supported` | `(country: str)` | `bool` |

**`generate` returns:**
```python
{
    "country_code": "US",
    "country_name": "United States",
    "name": "Passport",
    "code": "123456789",
    "valid": True
}
```

---

### Functions

| Function | Signature | Returns |
|----------|-----------|---------|
| `generate_iban` | `(country: str \| None = None)` | `str` |
| `validate_iban` | `(iban: str)` | `bool` |
| `format_iban` | `(iban: str)` | `str` |
| `iban_countries` | `()` | `list[str]` |
