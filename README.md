# EU Test Data Generator

Generate valid, checksum-correct **IBANs** and **personal ID codes** for European and worldwide countries. Built for QA engineers, developers, and testers who need realistic test data without using real personal information.

## Features

- **96 IBAN countries** — full IBAN registry coverage with correct BBAN national checksums
- **31 personal ID formats** — EE Isikukood, FI HETU, SE Personnummer, NO Fodselsnummer, DE Steuerliche IdNr, IT Codice Fiscale, PL PESEL, and 24 more
- **Single binary** — no runtime dependencies, instant startup
- **Library + CLI** — use as a Rust crate or a standalone command-line tool
- **Checksum-validated** — every generated code passes mod-97 (IBAN) and national checksum algorithms
- **Gender & year filters** — generate personal IDs for specific demographics
- **CSV export** — output to stdout or directly to a file with `--csv [path]`

## Installation

### From source

```bash
cargo install --path .
```

### Build from GitHub

```bash
git clone https://github.com/Sunyata-OU/EU-Test-Data-Generator.git
cd EU-Test-Data-Generator
cargo build --release
# Binary at target/release/eu-test-data-generator
```

### As a library dependency

```toml
[dependencies]
eu-test-data-generator = { git = "https://github.com/Sunyata-OU/EU-Test-Data-Generator.git", default-features = false }

# With CSV formatting helpers
eu-test-data-generator = { git = "https://github.com/Sunyata-OU/EU-Test-Data-Generator.git", default-features = false, features = ["csv"] }
```

## CLI Usage

### IBAN Generation

```bash
# Generate a random IBAN (any country)
eu-test-data-generator iban

# Generate 5 German IBANs
eu-test-data-generator iban DE 5

# Generate 10 random IBANs
eu-test-data-generator iban 10

# Export as CSV to stdout
eu-test-data-generator iban DE 5 --csv

# Export as CSV to a file
eu-test-data-generator iban DE 100 --csv ibans.csv
```

**Output:**
```
$ eu-test-data-generator iban DE 5
DE61 5152 4598 9311 1486 94  (valid: True)
DE74 6764 2027 4349 0222 77  (valid: True)
DE46 3745 8558 5637 9880 16  (valid: True)
DE22 9144 1329 8459 5887 39  (valid: True)
DE20 5970 6928 8450 8215 16  (valid: True)
```

```
$ eu-test-data-generator iban 5
LI08 1866 5KVP TE0V UZ8N K  (valid: True)
YT29 6896 5689 50I2 PKA5 1CSF Z34  (valid: True)
EG50 7105 0758 4025 6550 2608 9641 4  (valid: True)
AX02 8462 5859 7927 10  (valid: True)
AE21 4623 7720 3406 6426 400  (valid: True)
```

**CSV output:**
```
$ eu-test-data-generator iban DE 3 --csv
country,iban,iban_formatted,valid
DE,DE72902438106569462453,DE72 9024 3810 6569 4624 53,true
DE,DE86958518594489801701,DE86 9585 1859 4489 8017 01,true
DE,DE85188047151769010717,DE85 1880 4715 1769 0107 17,true
```

### Personal ID Generation

```bash
# Generate an Estonian Isikukood
eu-test-data-generator id

# Generate 3 Finnish HETUs for females
eu-test-data-generator id 3 --country=FI --gender=f

# Generate Italian Codice Fiscale
eu-test-data-generator id 3 --country=IT

# List all supported ID formats
eu-test-data-generator id --list

# Export as CSV to stdout
eu-test-data-generator id 5 --country=EE --csv

# Export as CSV to a file
eu-test-data-generator id 100 --country=FI --gender=f --csv finnish_ids.csv
```

**Output:**
```
$ eu-test-data-generator id 3 --country=FI --gender=f
FI - Henkilotunnus:
  050497-598S  (female, 1997-04-05, valid: True)
  160545-2224  (female, 1945-05-16, valid: True)
  050492-3084  (female, 1992-04-05, valid: True)
```

```
$ eu-test-data-generator id 3 --country=IT
IT - Codice Fiscale:
  ZDGQJT65R51C855N  (female, 1965-10-11, valid: True)
  WXVXRX67M13M412C  (male, 1967-08-13, valid: True)
  NDYXSQ43D60F707H  (female, 1943-04-20, valid: True)
```

**CSV output:**
```
$ eu-test-data-generator id 3 --country=FI --csv
country,id_name,code,gender,dob,valid
FI,Henkilotunnus,150454-612K,female,1954-04-15,true
FI,Henkilotunnus,170171-328N,female,1971-01-17,true
FI,Henkilotunnus,011142-293T,male,1942-11-01,true
```

```
$ eu-test-data-generator id --list
Code   ID Name
-------------------------------
AT     Sozialversicherungsnr
BA     JMBG
BE     Rijksregisternr
BG     EGN
CH     AHV-Nummer
CZ     Rodne cislo
DE     Steuerliche IdNr
DK     CPR-nummer
EE     Isikukood
ES     DNI
FI     Henkilotunnus
FR     NIR
GB     NINO
GR     AMKA
HR     OIB
IE     PPS Number
IS     Kennitala
IT     Codice Fiscale
LT     Asmens kodas
LV     Personas kods
ME     JMBG
NL     BSN
NO     Fodselsnummer
PL     PESEL
PT     NIF
RO     CNP
RS     JMBG
SE     Personnummer
SI     EMSO
SK     Rodne cislo
TR     TC Kimlik No
```

## Library Usage

Use as a Rust library by adding it as a dependency with `default-features = false` (this excludes `clap`, so only `rand` is pulled in).

```rust
use rand::thread_rng;
use eu_test_data_generator::{iban, personal_id};

fn main() {
    let mut rng = thread_rng();

    // Generate a German IBAN
    let code = iban::generate_iban(Some("DE"), &mut rng).unwrap();
    println!("{}", iban::format_iban(&code));
    // => DE89 3704 0044 0532 0130 00

    // Validate an IBAN
    assert!(iban::validate_iban(&code));

    // Generate a random IBAN (any country)
    let random = iban::generate_iban(None, &mut rng).unwrap();
    println!("{}", iban::format_iban(&random));

    // Generate an Estonian personal ID
    let registry = personal_id::Registry::new();
    let opts = personal_id::GenOptions::default();
    let id = registry.generate("EE", &opts, &mut rng).unwrap();
    println!("{}", id);

    // Generate a female Finnish HETU
    let opts = personal_id::GenOptions {
        gender: Some(personal_id::date::Gender::Female),
        year: Some(1990),
    };
    let fi_id = registry.generate("FI", &opts, &mut rng).unwrap();
    let parsed = registry.parse("FI", &fi_id).unwrap();
    println!("{} ({})", parsed.code, parsed.gender.unwrap());
}
```

### Available API

```rust
// IBAN
iban::generate_iban(country: Option<&str>, rng) -> Result<String, String>
iban::validate_iban(iban: &str) -> bool
iban::format_iban(iban: &str) -> String
iban::supported_countries() -> Vec<&'static str>

// Personal ID
personal_id::Registry::new() -> Registry
registry.generate(country, &opts, rng) -> Option<String>
registry.validate(country, code) -> Option<bool>
registry.parse(country, code) -> Option<IdResult>
registry.list_countries() -> Vec<(&str, &str)>

// CSV (requires "csv" feature)
csv::IBAN_HEADER                                          // "country,iban,iban_formatted,valid"
csv::ID_HEADER                                            // "country,id_name,code,gender,dob,valid"
csv::iban_row(iban: &str, formatted: &str, valid: bool) -> String
csv::id_row(country: &str, id_name: &str, result: &IdResult) -> String
csv::write_iban_csv(writer: &mut W, rows: &[(String, String, bool)]) -> io::Result<()>
csv::write_id_csv(writer: &mut W, country: &str, id_name: &str, results: &[IdResult]) -> io::Result<()>
```

## Supported Countries

### IBANs (96 countries)

AD, AE, AL, AT, AX, AZ, BA, BE, BG, BH, BR, BY, CH, CR, CY, CZ, DE, DK, DO, EE, EG, ES, FI, FO, FR, GB, GE, GF, GI, GL, GP, GR, GT, HR, HU, IE, IL, IQ, IS, IT, JO, KW, KZ, LB, LC, LI, LT, LU, LV, LY, MC, MD, ME, MF, MK, MN, MQ, MR, MT, MU, NC, NI, NL, NO, OM, PF, PK, PL, PM, PS, PT, QA, RE, RO, RS, RU, SA, SC, SD, SE, SI, SK, SM, SO, ST, SV, TF, TL, TN, TR, UA, VA, VG, WF, XK, YT

Countries with BBAN national checksums (17): BA, BE, CZ, EE, ES, FR, GF, GP, HR, HU, MC, MF, MK, MQ, NC, NO, PF, PL, PM, PT, RE, RS, SI, SK, TF, WF, YT

### Personal IDs (31 country codes)

| Country | Code | ID Name | Format |
|---------|------|---------|--------|
| Austria | AT | Sozialversicherungsnr | 10 digits |
| Belgium | BE | Rijksregisternr | YY.MM.DD-NNN.CC |
| Bosnia | BA | JMBG | 13 digits |
| Bulgaria | BG | EGN | 10 digits |
| Croatia | HR | OIB | 11 digits |
| Czech Republic | CZ | Rodne cislo | YYMMDD/XXXX |
| Denmark | DK | CPR-nummer | DDMMYY-SSSS |
| Estonia | EE | Isikukood | 11 digits |
| Finland | FI | Henkilotunnus | DDMMYY-NNNC |
| France | FR | NIR | 15 digits |
| Germany | DE | Steuerliche IdNr | 11 digits |
| Greece | GR | AMKA | 11 digits |
| Iceland | IS | Kennitala | 10 digits |
| Ireland | IE | PPS Number | 7 digits + 2 letters |
| Italy | IT | Codice Fiscale | 16 alphanumeric |
| Latvia | LV | Personas kods | 11 digits |
| Lithuania | LT | Asmens kodas | 11 digits |
| Montenegro | ME | JMBG | 13 digits |
| Netherlands | NL | BSN | 9 digits |
| Norway | NO | Fodselsnummer | 11 digits |
| Poland | PL | PESEL | 11 digits |
| Portugal | PT | NIF | 9 digits |
| Romania | RO | CNP | 13 digits |
| Serbia | RS | JMBG | 13 digits |
| Slovakia | SK | Rodne cislo | YYMMDD/XXXX |
| Slovenia | SI | EMSO | 13 digits |
| Spain | ES | DNI | 8 digits + letter |
| Sweden | SE | Personnummer | YYMMDD-NNNN |
| Switzerland | CH | AHV-Nummer | 756.XXXX.XXXX.XX |
| Turkey | TR | TC Kimlik No | 11 digits |
| United Kingdom | GB | NINO | 2 letters + 6 digits + letter |

## Validation

All generated codes are verified against established validation libraries:

- **IBANs**: 100% pass rate against [ibantools](https://www.npmjs.com/package/ibantools) (960/960, including BBAN national checksums)
- **Personal IDs**: 100% pass rate against [stdnum-js](https://github.com/koblas/stdnum-js) (3000/3000 across 30 countries)

## Project Structure

```
src/
  lib.rs                   -- Library entry point
  main.rs                  -- CLI (clap)
  iban/
    mod.rs                 -- Public API re-exports
    types.rs               -- CharType, BbanField, CountryFormat
    countries.rs           -- 96 country BBAN format definitions
    checksum.rs            -- BBAN national checksum algorithms
    generate.rs            -- IBAN generation, formatting, validation
    util.rs                -- Random character helpers
  personal_id/
    mod.rs                 -- Registry, GenOptions, IdResult
    checksum.rs            -- Luhn, ISO 7064, weighted checksums
    date.rs                -- Gender, date utilities
    ee.rs .. tr.rs         -- 26 country-specific implementations
```

## License

MIT
