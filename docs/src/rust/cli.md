# CLI Usage

## Installation

```bash
cargo install idsmith
```

## Validating Data

Use the `validate` command to check if a code is checksum and format correct.

```bash
# Validate an IBAN
idsmith validate iban DE47508562162522867909

# Validate a National ID (requires country code)
idsmith validate id 446-72-2445 --country US

# Validate a Credit Card
idsmith validate card 5590133141634919

# Validate a Bank Account
idsmith validate account 167078019952865929 --country MX

# Validate a SWIFT/BIC code
idsmith validate swift PBIHNLY9XXX

# Validate a Driver's License
idsmith validate license A123456789012 --country US

# Validate a Tax ID
idsmith validate tax ABCDE1234F --country IN

# Validate a Passport
idsmith validate passport 123456789 --country US
```

## Generating Data

Generate any identifier using subcommands. Use the optional count positional argument.

```bash
# Generate 5 German IBANs
idsmith iban DE 5

# Generate 3 US Bank Accounts in JSON
idsmith account 3 --country US --json -

# Generate a random Credit Card
idsmith card --brand amex

# Generate 10 Brazilian personal IDs
idsmith id 10 --country BR

# Generate Company IDs
idsmith company 5 --country GB

# Generate SWIFT codes
idsmith swift 3 --country US

# Generate 5 US Driver's Licenses
idsmith license 5 --country US

# Generate 3 Indian Tax IDs (PAN)
idsmith tax 3 --country IN

# Generate 10 German Passports
idsmith passport 10 --country DE
```

## Output Formats

```bash
# Default: plain text (one per line)
idsmith iban DE 5

# JSON output
idsmith iban DE 5 --json -

# CSV output
idsmith iban DE 5 --csv -

# JSON to file
idsmith iban DE 100 --json output.json

# CSV to file
idsmith iban DE 100 --csv output.csv
```

## Filtering Options

```bash
# Personal ID with gender filter
idsmith id 5 --country EE --gender male

# Personal ID with year filter
idsmith id 5 --country EE --year 1990

# Bank account with bank code
idsmith account 5 --country US --bank-code 021000021

# Credit card by brand
idsmith card 5 --brand visa
```
