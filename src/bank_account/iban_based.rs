use rand::Rng;

use super::AccountResult;

struct IbanCountryInfo {
    code: &'static str,
    country_name: &'static str,
}

static IBAN_COUNTRIES: &[IbanCountryInfo] = &[
    IbanCountryInfo {
        code: "AD",
        country_name: "Andorra",
    },
    IbanCountryInfo {
        code: "AE",
        country_name: "United Arab Emirates",
    },
    IbanCountryInfo {
        code: "AL",
        country_name: "Albania",
    },
    IbanCountryInfo {
        code: "AT",
        country_name: "Austria",
    },
    IbanCountryInfo {
        code: "AX",
        country_name: "Aland Islands",
    },
    IbanCountryInfo {
        code: "AZ",
        country_name: "Azerbaijan",
    },
    IbanCountryInfo {
        code: "BA",
        country_name: "Bosnia and Herzegovina",
    },
    IbanCountryInfo {
        code: "BE",
        country_name: "Belgium",
    },
    IbanCountryInfo {
        code: "BG",
        country_name: "Bulgaria",
    },
    IbanCountryInfo {
        code: "BH",
        country_name: "Bahrain",
    },
    IbanCountryInfo {
        code: "BR",
        country_name: "Brazil",
    },
    IbanCountryInfo {
        code: "BY",
        country_name: "Belarus",
    },
    IbanCountryInfo {
        code: "CH",
        country_name: "Switzerland",
    },
    IbanCountryInfo {
        code: "CR",
        country_name: "Costa Rica",
    },
    IbanCountryInfo {
        code: "CY",
        country_name: "Cyprus",
    },
    IbanCountryInfo {
        code: "CZ",
        country_name: "Czech Republic",
    },
    IbanCountryInfo {
        code: "DE",
        country_name: "Germany",
    },
    IbanCountryInfo {
        code: "DK",
        country_name: "Denmark",
    },
    IbanCountryInfo {
        code: "DO",
        country_name: "Dominican Republic",
    },
    IbanCountryInfo {
        code: "EE",
        country_name: "Estonia",
    },
    IbanCountryInfo {
        code: "EG",
        country_name: "Egypt",
    },
    IbanCountryInfo {
        code: "ES",
        country_name: "Spain",
    },
    IbanCountryInfo {
        code: "FI",
        country_name: "Finland",
    },
    IbanCountryInfo {
        code: "FO",
        country_name: "Faroe Islands",
    },
    IbanCountryInfo {
        code: "FR",
        country_name: "France",
    },
    IbanCountryInfo {
        code: "GB",
        country_name: "United Kingdom",
    },
    IbanCountryInfo {
        code: "GE",
        country_name: "Georgia",
    },
    IbanCountryInfo {
        code: "GF",
        country_name: "French Guiana",
    },
    IbanCountryInfo {
        code: "GI",
        country_name: "Gibraltar",
    },
    IbanCountryInfo {
        code: "GL",
        country_name: "Greenland",
    },
    IbanCountryInfo {
        code: "GP",
        country_name: "Guadeloupe",
    },
    IbanCountryInfo {
        code: "GR",
        country_name: "Greece",
    },
    IbanCountryInfo {
        code: "GT",
        country_name: "Guatemala",
    },
    IbanCountryInfo {
        code: "HR",
        country_name: "Croatia",
    },
    IbanCountryInfo {
        code: "HU",
        country_name: "Hungary",
    },
    IbanCountryInfo {
        code: "IE",
        country_name: "Ireland",
    },
    IbanCountryInfo {
        code: "IL",
        country_name: "Israel",
    },
    IbanCountryInfo {
        code: "IQ",
        country_name: "Iraq",
    },
    IbanCountryInfo {
        code: "IS",
        country_name: "Iceland",
    },
    IbanCountryInfo {
        code: "IT",
        country_name: "Italy",
    },
    IbanCountryInfo {
        code: "JO",
        country_name: "Jordan",
    },
    IbanCountryInfo {
        code: "KW",
        country_name: "Kuwait",
    },
    IbanCountryInfo {
        code: "KZ",
        country_name: "Kazakhstan",
    },
    IbanCountryInfo {
        code: "LB",
        country_name: "Lebanon",
    },
    IbanCountryInfo {
        code: "LC",
        country_name: "Saint Lucia",
    },
    IbanCountryInfo {
        code: "LI",
        country_name: "Liechtenstein",
    },
    IbanCountryInfo {
        code: "LT",
        country_name: "Lithuania",
    },
    IbanCountryInfo {
        code: "LU",
        country_name: "Luxembourg",
    },
    IbanCountryInfo {
        code: "LV",
        country_name: "Latvia",
    },
    IbanCountryInfo {
        code: "LY",
        country_name: "Libya",
    },
    IbanCountryInfo {
        code: "MC",
        country_name: "Monaco",
    },
    IbanCountryInfo {
        code: "MD",
        country_name: "Moldova",
    },
    IbanCountryInfo {
        code: "ME",
        country_name: "Montenegro",
    },
    IbanCountryInfo {
        code: "MF",
        country_name: "Saint Martin",
    },
    IbanCountryInfo {
        code: "MK",
        country_name: "North Macedonia",
    },
    IbanCountryInfo {
        code: "MN",
        country_name: "Mongolia",
    },
    IbanCountryInfo {
        code: "MQ",
        country_name: "Martinique",
    },
    IbanCountryInfo {
        code: "MR",
        country_name: "Mauritania",
    },
    IbanCountryInfo {
        code: "MT",
        country_name: "Malta",
    },
    IbanCountryInfo {
        code: "MU",
        country_name: "Mauritius",
    },
    IbanCountryInfo {
        code: "NC",
        country_name: "New Caledonia",
    },
    IbanCountryInfo {
        code: "NI",
        country_name: "Nicaragua",
    },
    IbanCountryInfo {
        code: "NL",
        country_name: "Netherlands",
    },
    IbanCountryInfo {
        code: "NO",
        country_name: "Norway",
    },
    IbanCountryInfo {
        code: "OM",
        country_name: "Oman",
    },
    IbanCountryInfo {
        code: "PF",
        country_name: "French Polynesia",
    },
    IbanCountryInfo {
        code: "PK",
        country_name: "Pakistan",
    },
    IbanCountryInfo {
        code: "PL",
        country_name: "Poland",
    },
    IbanCountryInfo {
        code: "PM",
        country_name: "Saint Pierre and Miquelon",
    },
    IbanCountryInfo {
        code: "PS",
        country_name: "Palestine",
    },
    IbanCountryInfo {
        code: "PT",
        country_name: "Portugal",
    },
    IbanCountryInfo {
        code: "QA",
        country_name: "Qatar",
    },
    IbanCountryInfo {
        code: "RE",
        country_name: "Reunion",
    },
    IbanCountryInfo {
        code: "RO",
        country_name: "Romania",
    },
    IbanCountryInfo {
        code: "RS",
        country_name: "Serbia",
    },
    IbanCountryInfo {
        code: "RU",
        country_name: "Russia",
    },
    IbanCountryInfo {
        code: "SA",
        country_name: "Saudi Arabia",
    },
    IbanCountryInfo {
        code: "SC",
        country_name: "Seychelles",
    },
    IbanCountryInfo {
        code: "SD",
        country_name: "Sudan",
    },
    IbanCountryInfo {
        code: "SE",
        country_name: "Sweden",
    },
    IbanCountryInfo {
        code: "SI",
        country_name: "Slovenia",
    },
    IbanCountryInfo {
        code: "SK",
        country_name: "Slovakia",
    },
    IbanCountryInfo {
        code: "SM",
        country_name: "San Marino",
    },
    IbanCountryInfo {
        code: "SO",
        country_name: "Somalia",
    },
    IbanCountryInfo {
        code: "ST",
        country_name: "Sao Tome and Principe",
    },
    IbanCountryInfo {
        code: "SV",
        country_name: "El Salvador",
    },
    IbanCountryInfo {
        code: "TF",
        country_name: "French Southern Territories",
    },
    IbanCountryInfo {
        code: "TL",
        country_name: "Timor-Leste",
    },
    IbanCountryInfo {
        code: "TN",
        country_name: "Tunisia",
    },
    IbanCountryInfo {
        code: "TR",
        country_name: "Turkey",
    },
    IbanCountryInfo {
        code: "UA",
        country_name: "Ukraine",
    },
    IbanCountryInfo {
        code: "VA",
        country_name: "Vatican City",
    },
    IbanCountryInfo {
        code: "VG",
        country_name: "British Virgin Islands",
    },
    IbanCountryInfo {
        code: "WF",
        country_name: "Wallis and Futuna",
    },
    IbanCountryInfo {
        code: "XK",
        country_name: "Kosovo",
    },
    IbanCountryInfo {
        code: "YT",
        country_name: "Mayotte",
    },
];

fn find_info(code: &str) -> Option<&'static IbanCountryInfo> {
    IBAN_COUNTRIES.iter().find(|c| c.code == code)
}

pub fn is_supported(code: &str) -> bool {
    find_info(code).is_some()
}

pub fn generate(code: &str, rng: &mut impl Rng) -> Option<AccountResult> {
    let info = find_info(code)?;
    let fields = crate::iban::get_format(code)?;
    let iban = crate::iban::generate_iban(Some(code), rng).ok()?;
    let bban = &iban[4..];

    let first_len = fields[0].length as usize;
    let bank_code = bban[..first_len].to_string();

    let (branch_code, acct_start) = if fields.len() >= 3 {
        let second_len = fields[1].length as usize;
        (
            Some(bban[first_len..first_len + second_len].to_string()),
            first_len + second_len,
        )
    } else {
        (None, first_len)
    };

    let account = bban[acct_start..].to_string();
    let formatted = crate::iban::format_iban(&iban);

    Some(AccountResult {
        country_code: code.to_string(),
        country_name: info.country_name.to_string(),
        format_name: "IBAN Account".to_string(),
        bank_code: Some(bank_code),
        branch_code,
        account_number: account,
        check_digits: Some(iban[2..4].to_string()),
        formatted,
        raw: bban.to_string(),
        iban: Some(iban),
        valid: true,
    })
}

pub fn validate(code: &str, raw: &str) -> Option<bool> {
    if find_info(code).is_none() {
        return None;
    }
    let fields = crate::iban::get_format(code)?;
    let expected_len: u8 = fields.iter().map(|f| f.length).sum();
    if raw.len() != expected_len as usize {
        return Some(false);
    }
    // Validate character types per BBAN field
    let mut offset = 0usize;
    for field in fields {
        let len = field.length as usize;
        let segment = &raw[offset..offset + len];
        let ok = match field.char_type {
            crate::iban::CharType::Numeric => segment.chars().all(|c| c.is_ascii_digit()),
            crate::iban::CharType::Alpha => segment.chars().all(|c| c.is_ascii_uppercase()),
            crate::iban::CharType::Alphanumeric => {
                segment.chars().all(|c| c.is_ascii_digit() || c.is_ascii_uppercase())
            }
        };
        if !ok {
            return Some(false);
        }
        offset += len;
    }
    Some(true)
}

pub fn format(code: &str, raw: &str) -> Option<String> {
    if find_info(code).is_none() {
        return None;
    }
    Some(raw.to_string())
}

pub fn list_countries() -> Vec<(&'static str, &'static str, &'static str, bool)> {
    IBAN_COUNTRIES
        .iter()
        .map(|c| (c.code, c.country_name, "IBAN Account", true))
        .collect()
}
