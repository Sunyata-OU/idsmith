# idsmith

Forge valid, checksum-correct **IBANs**, **personal IDs**, and **bank accounts** for 252 countries. Built for QA engineers, developers, and testers who need realistic test data without using real personal information.

## Features

- **96 IBAN countries** — full IBAN registry coverage with correct BBAN national checksums
- **252 bank account formats** — US ABA routing, MX CLABE, AU BSB, IN IFSC, AR CBU, NG NUBAN, BR mod-11, all IBAN countries, 120+ generic countries, and 22 territory aliases — covering every ISO 3166-1 country code
- **252 personal ID formats** — 56 country-specific implementations with real checksums (Luhn, Verhoeff, ISO 7064, mod-11, weighted), 160+ generic countries, and 34 territory aliases — matching bank account coverage
- **Single binary** — no runtime dependencies, instant startup
- **Library + CLI** — use as a Rust crate or a standalone command-line tool
- **Checksum-validated** — every generated code passes mod-97 (IBAN), ABA, CLABE, Luhn, Verhoeff, ISO 7064, CBU, NUBAN, and national checksum algorithms
- **Gender & year filters** — generate personal IDs for specific demographics
- **CSV export** — RFC 4180 compliant output to stdout or directly to a file with `--csv [path]`

## Installation

### From source

```bash
cargo install --path .
```

### Build from GitHub

```bash
git clone https://github.com/Sunyata-OU/idsmith.git
cd idsmith
cargo build --release
# Binary at target/release/idsmith
```

### As a library dependency

```toml
[dependencies]
idsmith = { git = "https://github.com/Sunyata-OU/idsmith.git", default-features = false }

# With CSV formatting helpers
idsmith = { git = "https://github.com/Sunyata-OU/idsmith.git", default-features = false, features = ["csv"] }
```

## CLI Usage

### IBAN Generation

```bash
# Generate a random IBAN (any country)
idsmith iban

# Generate 5 German IBANs
idsmith iban DE 5

# Generate 10 random IBANs
idsmith iban 10

# Export as CSV to stdout
idsmith iban DE 5 --csv

# Export as CSV to a file
idsmith iban DE 100 --csv ibans.csv
```

**Output:**
```
$ idsmith iban DE 3
DE61 5152 4598 9311 1486 94  (valid: True)
DE74 6764 2027 4349 0222 77  (valid: True)
DE46 3745 8558 5637 9880 16  (valid: True)
```

### Bank Account Generation

```bash
# Generate a random bank account (any country)
idsmith account

# Generate 5 US bank accounts
idsmith account 5 --country=US

# Generate a Mexican CLABE
idsmith account --country=MX

# List all supported countries
idsmith account --list

# Export as CSV to stdout
idsmith account 5 --country=US --csv

# Export as CSV to a file
idsmith account 100 --country=AU --csv accounts.csv
```

**Output** (shows all fields needed for testing):
```
$ idsmith account --country=US 3
US - United States - ABA Routing + Account:
  Bank: 101921606 | Account: 083312379059223 | Check: 6 | Formatted: 101921606 083312379059223 | Raw: 101921606083312379059223 | valid: True
US - United States - ABA Routing + Account:
  Bank: 073120784 | Account: 27803808483813 | Check: 4 | Formatted: 073120784 27803808483813 | Raw: 07312078427803808483813 | valid: True
US - United States - ABA Routing + Account:
  Bank: 122125180 | Account: 941518071150 | Check: 0 | Formatted: 122125180 941518071150 | Raw: 122125180941518071150 | valid: True
```

```
$ idsmith account --country=GB
GB - United Kingdom - Sort Code + Account:
  Bank: 366955 | Account: 93265548 | IBAN: GB30 ZBCJ 2650 2102 1520 51 | Formatted: 36-69-55 93265548 | Raw: 36695593265548 | valid: True
```

**CSV output** (RFC 4180 compliant with proper quoting):
```
$ idsmith account 3 --country=US --csv
country,country_name,format,bank_code,branch_code,account_number,check_digits,formatted,raw,iban,valid
US,United States,ABA Routing + Account,094214349,,13783165133275870,9,094214349 13783165133275870,09421434913783165133275870,,true
US,United States,ABA Routing + Account,076194050,,9816717869449,0,076194050 9816717869449,0761940509816717869449,,true
US,United States,ABA Routing + Account,085533796,,174463458438366,6,085533796 174463458438366,085533796174463458438366,,true
```

```
$ idsmith account --list
Code   Country                   Format                         IBAN
----------------------------------------------------------------------
AD     Andorra                   IBAN Account                   Yes
AE     United Arab Emirates      IBAN Account                   Yes
AF     Afghanistan               Bank + Account                 No
AR     Argentina                 CBU                            No
AS     American Samoa            ABA Routing + Account          No
AU     Australia                 BSB + Account                  No
...    (252 countries total)
US     United States             ABA Routing + Account          No
ZA     South Africa              Branch + Account               No
ZW     Zimbabwe                  Bank + Account                 No
```

### Personal ID Generation

```bash
# Generate an Estonian Isikukood
idsmith id

# Generate 3 Finnish HETUs for females
idsmith id 3 --country=FI --gender=f

# Generate Italian Codice Fiscale
idsmith id 3 --country=IT

# List all supported ID formats
idsmith id --list

# Export as CSV to stdout
idsmith id 5 --country=EE --csv

# Export as CSV to a file
idsmith id 100 --country=FI --gender=f --csv finnish_ids.csv
```

**Output:**
```
$ idsmith id 3 --country=FI --gender=f
FI - Henkilotunnus:
  050497-598S  (female, 1997-04-05, valid: True)
  160545-2224  (female, 1945-05-16, valid: True)
  050492-3084  (female, 1992-04-05, valid: True)
```

```
$ idsmith id --list
Code   Country                   ID Name
-------------------------------------------------------
AT     Austria                   Sozialversicherungsnr
BA     Bosnia and Herzegovina    JMBG
BE     Belgium                   Rijksregisternr
BR     Brazil                    CPF
CN     China                     Resident ID
...    (252 countries total)
US     United States             SSN
ZA     South Africa              SA ID
```

## Library Usage

Use as a Rust library by adding it as a dependency with `default-features = false` (this excludes `clap`, so only `rand` is pulled in).

```rust
use rand::thread_rng;
use idsmith::{iban, personal_id, bank_account};

fn main() {
    let mut rng = thread_rng();

    // Generate a German IBAN
    let code = iban::generate_iban(Some("DE"), &mut rng).unwrap();
    println!("{}", iban::format_iban(&code));
    // => DE89 3704 0044 0532 0130 00

    // Validate an IBAN
    assert!(iban::validate_iban(&code));

    // Generate a US bank account
    let registry = bank_account::Registry::new();
    let opts = bank_account::GenOptions::default();
    let result = registry.generate("US", &opts, &mut rng).unwrap();
    println!("Bank: {} Account: {}", result.bank_code.unwrap(), result.account_number);

    // Generate an Estonian personal ID
    let id_registry = personal_id::Registry::new();
    let id_opts = personal_id::GenOptions::default();
    let id = id_registry.generate("EE", &id_opts, &mut rng).unwrap();
    println!("{}", id);
}
```

### Available API

```rust
// IBAN
iban::generate_iban(country: Option<&str>, rng) -> Result<String, String>
iban::validate_iban(iban: &str) -> bool
iban::format_iban(iban: &str) -> String
iban::supported_countries() -> Vec<&'static str>

// Bank Account
bank_account::Registry::new() -> Registry
registry.generate(country, &opts, rng) -> Option<AccountResult>
registry.validate(country, raw) -> Option<bool>
registry.format(country, raw) -> Option<String>
registry.is_supported(country) -> bool
registry.list_countries() -> Vec<(&str, &str, &str, bool)>

// AccountResult fields:
//   country_code, country_name, format_name,
//   bank_code, branch_code, account_number, check_digits,
//   formatted, raw, iban, valid

// Personal ID
personal_id::Registry::new() -> Registry
registry.generate(country, &opts, rng) -> Option<String>
registry.validate(country, code) -> Option<bool>
registry.parse(country, code) -> Option<IdResult>
registry.list_countries() -> Vec<(&str, &str, &str)>

// CSV (requires "csv" feature, RFC 4180 compliant)
csv::IBAN_HEADER                                          // "country,iban,iban_formatted,valid"
csv::ID_HEADER                                            // "country,id_name,code,gender,dob,valid"
csv::ACCOUNT_HEADER                                       // "country,country_name,format,..."
csv::iban_row(iban: &str, formatted: &str, valid: bool) -> String
csv::id_row(country: &str, id_name: &str, result: &IdResult) -> String
csv::account_row(result: &AccountResult) -> String
csv::write_iban_csv(writer: &mut W, rows: &[(String, String, bool)]) -> io::Result<()>
csv::write_id_csv(writer: &mut W, country: &str, id_name: &str, results: &[IdResult]) -> io::Result<()>
csv::write_account_csv(writer: &mut W, results: &[AccountResult]) -> io::Result<()>
```

## Supported Countries

### IBANs (96 countries)

AD, AE, AL, AT, AX, AZ, BA, BE, BG, BH, BR, BY, CH, CR, CY, CZ, DE, DK, DO, EE, EG, ES, FI, FO, FR, GB, GE, GF, GI, GL, GP, GR, GT, HR, HU, IE, IL, IQ, IS, IT, JO, KW, KZ, LB, LC, LI, LT, LU, LV, LY, MC, MD, ME, MF, MK, MN, MQ, MR, MT, MU, NC, NI, NL, NO, OM, PF, PK, PL, PM, PS, PT, QA, RE, RO, RS, RU, SA, SC, SD, SE, SI, SK, SM, SO, ST, SV, TF, TL, TN, TR, UA, VA, VG, WF, XK, YT

Countries with BBAN national checksums (17): BA, BE, CZ, EE, ES, FR, GF, GP, HR, HU, MC, MF, MK, MQ, NC, NO, PF, PL, PM, PT, RE, RS, SI, SK, TF, WF, YT

### Bank Accounts (252 countries)

**16 countries with specific implementations and checksum validation:**

| Country | Code | Format | Checksum |
|---------|------|--------|----------|
| Argentina | AR | CBU (22 digits) | Mod-10 weighted (7,1,3,9) |
| Australia | AU | BSB (6) + Account (5-9) | — |
| Brazil | BR | Bank (3) + Branch (4+1) + Account (6-10+1) | Mod-11 weighted |
| Canada | CA | Institution (3) + Transit (5) + Account (7-12) | — |
| China | CN | 16-19 digits | Luhn |
| Hong Kong | HK | Bank (3) + Account (9-12) | — |
| India | IN | IFSC (11) + Account (9-18) | — |
| Japan | JP | Bank (4) + Branch (3) + Type (1) + Account (7) | — |
| Mexico | MX | CLABE (18 digits) | Weighted 3-7-1 mod-10 |
| New Zealand | NZ | Bank (2) + Branch (4) + Account (7) + Suffix (2-3) | Algo A/B/D/E/F/G/X weighted |
| Nigeria | NG | NUBAN (10 digits) | Mod-10 weighted (3,7,3) |
| Singapore | SG | Bank (4) + Branch (3) + Account (6-10) | — |
| South Africa | ZA | Branch (6) + Account (7-11) | — |
| South Korea | KR | 11-14 digits | — |
| United Kingdom | GB | Sort Code (6) + Account (8) | — (+ IBAN) |
| United States | US | ABA Routing (9) + Account (8-17) | ABA weighted 3-7-1 mod-10 |

**96 IBAN countries** — domestic BBAN decomposition + valid IBAN (all mod-97 validated), with per-field character type validation (Numeric/Alpha/Alphanumeric)

**120 generic countries** — configurable bank code + account format (Asia, Africa, Americas, Oceania, Caribbean)

**22 territory aliases** — territories mapped to their parent country's banking system:

| Territory | Uses | Territory | Uses |
|-----------|------|-----------|------|
| AS, GU, MP, PR, UM, VI | US banking | CC, CX, HM, NF | AU banking |
| CK, NU, PN, TK | NZ banking | BL | FR banking (IBAN) |
| GS, IO, SH | GB banking | BV, SJ | NO banking (IBAN) |
| EH | MA banking | AQ | Generic |

### Personal IDs (252 countries)

**56 countries with specific implementations and checksum validation:**

| Country | Code | ID Name | Checksum | Gender | DOB |
|---------|------|---------|----------|--------|-----|
| Argentina | AR | CUIL | Weighted mod-11 | Yes | — |
| Australia | AU | TFN | Weighted mod-11 | — | — |
| Austria | AT | Sozialversicherungsnr | Weighted | — | Yes |
| Belgium | BE | Rijksregisternr | Mod-97 | Yes | Yes |
| Bosnia | BA | JMBG | Mod-11 | Yes | Yes |
| Brazil | BR | CPF | Mod-11 two-pass | — | — |
| Bulgaria | BG | EGN | Weighted | Yes | Yes |
| Canada | CA | SIN | Luhn | — | — |
| Chile | CL | RUT | Mod-11 | — | — |
| China | CN | Resident ID | ISO 7064 MOD 11-2 | Yes | Yes |
| Colombia | CO | NIT | Weighted mod-11 | — | — |
| Croatia | HR | OIB | ISO 7064 | — | — |
| Czech Republic | CZ | Rodne cislo | Mod-11 | Yes | Yes |
| Denmark | DK | CPR-nummer | Weighted | — | Yes |
| Ecuador | EC | Cedula | Luhn-variant | — | — |
| Egypt | EG | National ID | Luhn | Yes | Yes |
| Estonia | EE | Isikukood | Weighted | Yes | Yes |
| Finland | FI | Henkilotunnus | Mod-31 | Yes | Yes |
| France | FR | NIR | Mod-97 | Yes | Yes |
| Germany | DE | Steuerliche IdNr | ISO 7064 | — | — |
| Greece | GR | AMKA | Luhn | Yes | Yes |
| Hong Kong | HK | HKID | Weighted mod-11 | — | — |
| Iceland | IS | Kennitala | Weighted | — | Yes |
| India | IN | Aadhaar | Verhoeff | — | — |
| Indonesia | ID | NIK | — | Yes | Yes |
| Ireland | IE | PPS Number | Weighted | — | — |
| Israel | IL | Teudat Zehut | Luhn | — | — |
| Italy | IT | Codice Fiscale | Weighted | Yes | Yes |
| Japan | JP | My Number | Weighted mod-11 | — | — |
| Latvia | LV | Personas kods | Weighted | — | Yes |
| Lithuania | LT | Asmens kodas | Weighted | Yes | Yes |
| Malaysia | MY | MyKad | — | Yes | Yes |
| Mexico | MX | CURP | Weighted mod-10 | Yes | Yes |
| Montenegro | ME | JMBG | Mod-11 | Yes | Yes |
| Netherlands | NL | BSN | Mod-11 | — | — |
| New Zealand | NZ | IRD | Weighted mod-11 | — | — |
| Norway | NO | Fodselsnummer | Weighted | Yes | Yes |
| Peru | PE | DNI | — | — | — |
| Poland | PL | PESEL | Weighted | Yes | Yes |
| Portugal | PT | NIF | Mod-11 | — | — |
| Romania | RO | CNP | Weighted | Yes | Yes |
| Serbia | RS | JMBG | Mod-11 | Yes | Yes |
| Singapore | SG | NRIC | Weighted mod-11 | — | — |
| Slovakia | SK | Rodne cislo | Mod-11 | Yes | Yes |
| Slovenia | SI | EMSO | Mod-11 | Yes | Yes |
| South Africa | ZA | SA ID | Luhn | Yes | Yes |
| South Korea | KR | RRN | Weighted mod-11 | Yes | Yes |
| Spain | ES | DNI | Mod-23 | — | — |
| Sweden | SE | Personnummer | Luhn | Yes | Yes |
| Switzerland | CH | AHV-Nummer | EAN-13 | — | — |
| Taiwan | TW | National ID | Weighted mod-10 | Yes | — |
| Thailand | TH | Citizen ID | Weighted mod-11 | — | — |
| Turkey | TR | TC Kimlik No | Weighted | — | — |
| United Kingdom | GB | NINO | Format | — | — |
| United States | US | SSN | Format | — | — |
| Uruguay | UY | CI | Weighted mod-10 | — | — |

**160+ generic countries** — random digits/alphanumeric of correct length with format validation

**34 territory aliases** — territories mapped to their parent country's ID system:

| Territory | Uses | Territory | Uses |
|-----------|------|-----------|------|
| AS, GU, MP, PR, UM, VI | US (SSN) | CC, CX, HM, NF | AU (TFN) |
| CK, NU, PN, TK | NZ (IRD) | BL, GF, GP, MF, MQ, NC, PF, PM, RE, TF, WF, YT | FR (NIR) |
| GS, IO, SH, VG | GB (NINO) | BV, SJ | NO (Fodselsnummer) |
| FO, GL | DK (CPR-nummer) | | |

## Validation

All generated codes are verified against established external validation libraries:

- **IBANs**: 100% pass rate against [ibantools](https://www.npmjs.com/package/ibantools) (960/960, including BBAN national checksums)
- **Personal IDs**: Validated against external libraries:
  - **BR CPF, AR CUIL, CL RUT, CO NIT, EC Cedula**: 100/100 each against [python-stdnum](https://github.com/arthurdejong/python-stdnum)
  - **CN Resident ID, IN Aadhaar, KR RRN, TH Citizen ID**: 100/100 each against [python-stdnum](https://github.com/arthurdejong/python-stdnum) (ISO 7064, Verhoeff, weighted checksums)
  - **ZA SA ID, IL Teudat Zehut, AU TFN, CA SIN, NZ IRD**: 100/100 each against [python-stdnum](https://github.com/arthurdejong/python-stdnum) (Luhn, weighted mod-11)
  - **US SSN, MX CURP**: 100/100 each against [python-stdnum](https://github.com/arthurdejong/python-stdnum) (format rules, weighted mod-10 with Ñ alphabet)
  - **TW National ID**: 100/100 against [taiwan-id](https://www.npmjs.com/package/taiwan-id) (letter-weighted mod-10)
  - **European IDs** (30 countries): 100% pass rate against [stdnum-js](https://github.com/koblas/stdnum-js)
  - All 252 countries pass round-trip validation (generate then validate)
- **Bank Accounts**: Validated against external libraries:
  - **US ABA routing**: 100/100 against [abavalidator](https://www.npmjs.com/package/abavalidator) (checksum verification)
  - **MX CLABE**: 100/100 against [clabe-validator](https://www.npmjs.com/package/clabe-validator) (checksum + real Banxico bank/city codes)
  - **AR CBU**: 100/100 against [python-stdnum](https://github.com/arthurdejong/python-stdnum) `ar.cbu` (mod-10 weighted checksum)
  - **NZ Bank Account**: 100/100 against [python-stdnum](https://github.com/arthurdejong/python-stdnum) `nz.bankaccount` (algorithm A/B/D/E/F/G/X checksums + real NZ Payments branch registry)
  - **CN Luhn**: 100/100 against standard Luhn algorithm (independent implementation)
  - **BR mod-11**: 100/100 against independent mod-11 weighted checksum implementation
  - **NG NUBAN**: 100/100 against independent mod-10 weighted (3,7,3) checksum implementation
  - **GB IBAN**: 100/100 against [ibantools](https://www.npmjs.com/package/ibantools) (mod-97 checksum)
  - **IN IFSC**: 100/100 format validation against [ifsc](https://www.npmjs.com/package/ifsc) patterns
  - **IBAN countries** (28 sampled x 10): 280/280 against [ibantools](https://www.npmjs.com/package/ibantools) (mod-97 checksum)
  - All 252 countries pass round-trip validation (generate then validate) with corrupt-digit rejection for checksum countries

## Project Structure

```
src/
  lib.rs                   -- Library entry point
  main.rs                  -- CLI (clap)
  csv.rs                   -- CSV formatting helpers (optional, RFC 4180)
  iban/
    mod.rs                 -- Public API re-exports
    types.rs               -- CharType, BbanField, CountryFormat
    countries.rs           -- 96 country BBAN format definitions
    checksum.rs            -- BBAN national checksum algorithms
    generate.rs            -- IBAN generation, formatting, validation
    util.rs                -- Random character helpers
  bank_account/
    mod.rs                 -- Registry, AccountResult, GenOptions, aliases
    checksum.rs            -- ABA, CLABE, Luhn checksums
    iban_based.rs          -- IBAN country handler (96 countries)
    generic.rs             -- Non-IBAN country handler (120 countries)
    us.rs .. ng.rs         -- 16 country-specific implementations
  personal_id/
    mod.rs                 -- Registry, GenOptions, IdResult, 34 territory aliases
    checksum.rs            -- Luhn, Verhoeff, ISO 7064, weighted checksums
    date.rs                -- Gender, date utilities
    generic.rs             -- Non-specific country handler (160+ countries)
    ee.rs .. za.rs         -- 56 country-specific implementations
tests/
  bank_account_tests.rs    -- 14 bank account tests
  iban_tests.rs            -- 8 IBAN tests
  personal_id_tests.rs     -- 10 personal ID tests
```

## Acknowledgments

Thanks to these excellent validation libraries that helped verify the correctness of generated test data:

- [ibantools](https://github.com/Simplify/ibantools) — IBAN and BBAN validation for 96 countries
- [python-stdnum](https://github.com/arthurdejong/python-stdnum) — Personal ID validation (BR CPF, CN RIC, IN Aadhaar, MX CURP, and 13 more) + bank account validation (AR CBU, NZ) (Arthur de Jong)
- [abavalidator](https://github.com/JamesEggers1/node-ABAValidator) — US ABA routing number checksum validation
- [clabe-validator](https://github.com/center-key/clabe-validator) — Mexican CLABE validation with Banxico bank/city registry
- [ifsc](https://github.com/razorpay/ifsc) — Indian IFSC code validation (Razorpay)
- [stdnum-js](https://github.com/koblas/stdnum-js) — European personal ID number validation across 30 countries
- [taiwan-id](https://www.npmjs.com/package/taiwan-id) — Taiwan National ID validation

## License

MIT
