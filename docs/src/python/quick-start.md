# Python Quick Start

## Installation

```bash
pip install idsmith
```

## Validation

```python
import idsmith

# IBAN
idsmith.validate_iban("DE47508562162522867909")  # True

# Personal ID
idsmith.PersonalId.validate("US", "446-72-2445")  # True

# Credit Card
idsmith.CreditCard.validate("4152839405126374")  # True

# Bank Account
idsmith.BankAccount.validate("US", "021000021-123456789")  # True

# Company ID
idsmith.CompanyId.validate("GB", "GB123456789")  # True

# Driver's License
idsmith.DriverLicense.validate("US", "A123456789012")  # True

# Passport
idsmith.Passport.validate("DE", "C01234567")  # True

# Tax ID (TIN)
idsmith.TaxId.validate("IN", "ABCDE1234F")  # True

# SWIFT/BIC
idsmith.Swift.validate("CHASGB2LXXX")  # True
```

## Generation

```python
import idsmith

# Generate an IBAN
iban = idsmith.generate_iban("DE")
print(iban)  # DE47508562162522867909

# Generate a bank account (returns a dict)
account = idsmith.BankAccount.generate("US")
print(account["formatted"])   # 021000021 | 123456789
print(account["country_code"])  # US

# Generate a personal ID with options
code = idsmith.PersonalId.generate("EE", gender="female", year=1990)
print(code)  # 49001011234

# Generate a credit card
card = idsmith.CreditCard.generate(brand="visa")
print(card["formatted"])  # 4152 8394 0512 6374
print(card["brand"])      # VISA

# Generate a company ID
company = idsmith.CompanyId.generate(country="GB")
print(company["code"])  # GB123456789

# Generate a SWIFT code
swift = idsmith.Swift.generate(country="US")
print(swift["code"])  # CHASUSU5XXX

# Generate a Driver's License
dl = idsmith.DriverLicense.generate(country="US")

# Generate a Passport
passport = idsmith.Passport.generate(country="DE")

# Generate a Tax ID (TIN)
tin = idsmith.TaxId.generate(country="IN")
```

## Parsing

```python
result = idsmith.PersonalId.parse("EE", "49001011234")
print(result["dob"])     # 1990-01-01
print(result["gender"])  # female
print(result["valid"])   # True
```

## Listing Supported Countries

```python
# IBAN countries
countries = idsmith.iban_countries()  # ["AD", "AE", "AL", ...]

# Bank account countries (returns list of dicts)
for c in idsmith.BankAccount.list_countries():
    print(f"{c['code']} - {c['name']} ({c['format']})")

# Personal ID countries
for c in idsmith.PersonalId.list_countries():
    print(f"{c['code']} - {c['country_name']} ({c['id_name']})")

# Credit card brands
brands = idsmith.CreditCard.list_brands()  # ["Visa", "Mastercard", ...]

# Check if a country is supported
idsmith.BankAccount.is_supported("US")  # True
idsmith.PersonalId.is_supported("EE")   # True
```

## Error Handling

Unsupported countries raise `ValueError`:

```python
try:
    idsmith.PersonalId.generate("XX")
except ValueError as e:
    print(e)  # Unsupported country: XX
```
