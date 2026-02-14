use rand::Rng;

use super::AccountResult;

struct IbanCountryInfo {
    code: &'static str,
}

static IBAN_COUNTRIES: &[IbanCountryInfo] = &[
    IbanCountryInfo {
        code: "AD",
    },
    IbanCountryInfo {
        code: "AE",
    },
    IbanCountryInfo {
        code: "AL",
    },
    IbanCountryInfo {
        code: "AO",
    },
    IbanCountryInfo {
        code: "AT",
    },
    IbanCountryInfo {
        code: "AX",
    },
    IbanCountryInfo {
        code: "AZ",
    },
    IbanCountryInfo {
        code: "BA",
    },
    IbanCountryInfo {
        code: "BE",
    },
    IbanCountryInfo {
        code: "BF",
    },
    IbanCountryInfo {
        code: "BG",
    },
    IbanCountryInfo {
        code: "BH",
    },
    IbanCountryInfo {
        code: "BI",
    },
    IbanCountryInfo {
        code: "BJ",
    },
    IbanCountryInfo {
        code: "BR",
    },
    IbanCountryInfo {
        code: "BY",
    },
    IbanCountryInfo {
        code: "CF",
    },
    IbanCountryInfo {
        code: "CG",
    },
    IbanCountryInfo {
        code: "CH",
    },
    IbanCountryInfo {
        code: "CI",
    },
    IbanCountryInfo {
        code: "CM",
    },
    IbanCountryInfo {
        code: "CV",
    },
    IbanCountryInfo {
        code: "CR",
    },
    IbanCountryInfo {
        code: "CY",
    },
    IbanCountryInfo {
        code: "CZ",
    },
    IbanCountryInfo {
        code: "DE",
    },
    IbanCountryInfo {
        code: "DJ",
    },
    IbanCountryInfo {
        code: "DK",
    },
    IbanCountryInfo {
        code: "DZ",
    },
    IbanCountryInfo {
        code: "DO",
    },
    IbanCountryInfo {
        code: "EE",
    },
    IbanCountryInfo {
        code: "EG",
    },
    IbanCountryInfo {
        code: "ES",
    },
    IbanCountryInfo {
        code: "FI",
    },
    IbanCountryInfo {
        code: "FO",
    },
    IbanCountryInfo {
        code: "FR",
    },
    IbanCountryInfo {
        code: "GA",
    },
    IbanCountryInfo {
        code: "GB",
    },
    IbanCountryInfo {
        code: "GE",
    },
    IbanCountryInfo {
        code: "GG",
    },
    IbanCountryInfo {
        code: "GF",
    },
    IbanCountryInfo {
        code: "GI",
    },
    IbanCountryInfo {
        code: "GL",
    },
    IbanCountryInfo {
        code: "GN",
    },
    IbanCountryInfo {
        code: "GP",
    },
    IbanCountryInfo {
        code: "GQ",
    },
    IbanCountryInfo {
        code: "GR",
    },
    IbanCountryInfo {
        code: "GT",
    },
    IbanCountryInfo {
        code: "GW",
    },
    IbanCountryInfo {
        code: "HR",
    },
    IbanCountryInfo {
        code: "HU",
    },
    IbanCountryInfo {
        code: "IE",
    },
    IbanCountryInfo {
        code: "IL",
    },
    IbanCountryInfo {
        code: "IM",
    },
    IbanCountryInfo {
        code: "IR",
    },
    IbanCountryInfo {
        code: "IQ",
    },
    IbanCountryInfo {
        code: "IS",
    },
    IbanCountryInfo {
        code: "IT",
    },
    IbanCountryInfo {
        code: "JE",
    },
    IbanCountryInfo {
        code: "JO",
    },
    IbanCountryInfo {
        code: "KM",
    },
    IbanCountryInfo {
        code: "KW",
    },
    IbanCountryInfo {
        code: "KZ",
    },
    IbanCountryInfo {
        code: "LB",
    },
    IbanCountryInfo {
        code: "MA",
    },
    IbanCountryInfo {
        code: "MG",
    },
    IbanCountryInfo {
        code: "ML",
    },
    IbanCountryInfo {
        code: "LC",
    },
    IbanCountryInfo {
        code: "LI",
    },
    IbanCountryInfo {
        code: "LT",
    },
    IbanCountryInfo {
        code: "LU",
    },
    IbanCountryInfo {
        code: "LV",
    },
    IbanCountryInfo {
        code: "LY",
    },
    IbanCountryInfo {
        code: "MC",
    },
    IbanCountryInfo {
        code: "MD",
    },
    IbanCountryInfo {
        code: "ME",
    },
    IbanCountryInfo {
        code: "MF",
    },
    IbanCountryInfo {
        code: "MK",
    },
    IbanCountryInfo {
        code: "MN",
    },
    IbanCountryInfo {
        code: "MQ",
    },
    IbanCountryInfo {
        code: "MR",
    },
    IbanCountryInfo {
        code: "MZ",
    },
    IbanCountryInfo {
        code: "MT",
    },
    IbanCountryInfo {
        code: "MU",
    },
    IbanCountryInfo {
        code: "NC",
    },
    IbanCountryInfo {
        code: "NE",
    },
    IbanCountryInfo {
        code: "NI",
    },
    IbanCountryInfo {
        code: "NL",
    },
    IbanCountryInfo {
        code: "NO",
    },
    IbanCountryInfo {
        code: "OM",
    },
    IbanCountryInfo {
        code: "PF",
    },
    IbanCountryInfo {
        code: "PK",
    },
    IbanCountryInfo {
        code: "PL",
    },
    IbanCountryInfo {
        code: "PM",
    },
    IbanCountryInfo {
        code: "PS",
    },
    IbanCountryInfo {
        code: "PT",
    },
    IbanCountryInfo {
        code: "QA",
    },
    IbanCountryInfo {
        code: "RE",
    },
    IbanCountryInfo {
        code: "RO",
    },
    IbanCountryInfo {
        code: "RS",
    },
    IbanCountryInfo {
        code: "RU",
    },
    IbanCountryInfo {
        code: "SA",
    },
    IbanCountryInfo {
        code: "SC",
    },
    IbanCountryInfo {
        code: "SD",
    },
    IbanCountryInfo {
        code: "SE",
    },
    IbanCountryInfo {
        code: "SI",
    },
    IbanCountryInfo {
        code: "SK",
    },
    IbanCountryInfo {
        code: "SM",
    },
    IbanCountryInfo {
        code: "SN",
    },
    IbanCountryInfo {
        code: "SO",
    },
    IbanCountryInfo {
        code: "ST",
    },
    IbanCountryInfo {
        code: "SV",
    },
    IbanCountryInfo {
        code: "TD",
    },
    IbanCountryInfo {
        code: "TF",
    },
    IbanCountryInfo {
        code: "TG",
    },
    IbanCountryInfo {
        code: "TL",
    },
    IbanCountryInfo {
        code: "TN",
    },
    IbanCountryInfo {
        code: "TR",
    },
    IbanCountryInfo {
        code: "UA",
    },
    IbanCountryInfo {
        code: "VA",
    },
    IbanCountryInfo {
        code: "VG",
    },
    IbanCountryInfo {
        code: "WF",
    },
    IbanCountryInfo {
        code: "XK",
    },
    IbanCountryInfo {
        code: "YT",
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
        country_name: crate::countries::get_country_name(code).unwrap_or("Unknown").to_string(),
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
    find_info(code)?;
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
            crate::iban::CharType::Alphanumeric => segment
                .chars()
                .all(|c| c.is_ascii_digit() || c.is_ascii_uppercase()),
        };
        if !ok {
            return Some(false);
        }
        offset += len;
    }
    Some(true)
}

pub fn format(code: &str, raw: &str) -> Option<String> {
    find_info(code)?;
    Some(raw.to_string())
}

pub fn list_countries() -> Vec<(&'static str, &'static str, &'static str, bool)> {
    IBAN_COUNTRIES
        .iter()
        .map(|c| (c.code, crate::countries::get_country_name(c.code).unwrap_or("Unknown"), "IBAN Account", true))
        .collect()
}
