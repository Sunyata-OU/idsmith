# EU Test Data Generator

Generate valid, checksum-correct **IBANs** and **personal ID codes** for European and worldwide countries. Built for QA engineers, developers, and testers who need realistic test data without using real personal information.

## Features

- **96 IBAN countries** — full IBAN registry coverage with correct BBAN national checksums
- **31 personal ID formats** — EE Isikukood, FI HETU, SE Personnummer, NO Fodselsnummer, DE Steuerliche IdNr, IT Codice Fiscale, PL PESEL, and 24 more
- **Single binary** — no runtime dependencies, instant startup
- **Checksum-validated** — every generated code passes mod-97 (IBAN) and national checksum algorithms
- **Gender & year filters** — generate personal IDs for specific demographics

## Installation

### From source

```bash
cargo install --path .
```

### Build from GitHub

```bash
git clone https://github.com/Sunyata-OU/EU-Test-Data-Generator.git
cd eu-test-data-generator
cargo build --release
# Binary at target/release/eu-test-data-generator
```

## Usage

### IBAN Generation

```bash
# Generate a random IBAN (any country)
eu-test-data-generator iban

# Generate 5 German IBANs
eu-test-data-generator iban DE 5

# Generate 10 random IBANs
eu-test-data-generator iban 10
```

**Example output:**
```
DE89 3704 0044 0532 0130 00  (valid: True)
FR76 3000 6000 01IF 8LB2 V0V0 P74  (valid: True)
EE38 2200 2210 2014 5685  (valid: True)
```

### Personal ID Generation

```bash
# Generate an Estonian Isikukood
eu-test-data-generator id

# Generate 5 Finnish HETUs for females
eu-test-data-generator id 5 --country=FI --gender=f

# Generate Italian Codice Fiscale for birth year 1990
eu-test-data-generator id 3 --country=IT --year=1990

# List all supported ID formats
eu-test-data-generator id --list
```

**Example output:**
```
FI - Henkilotunnus:
  010385-482N  (female, 1985-03-01, valid: True)
  250892-694P  (female, 1992-08-25, valid: True)
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

## License

MIT
