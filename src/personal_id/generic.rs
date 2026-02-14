use rand::Rng;

struct GenericIdInfo {
    code: &'static str,
    country_name: &'static str,
    id_name: &'static str,
    length: u8,
    numeric_only: bool,
}

static GENERIC_COUNTRIES: &[GenericIdInfo] = &[
    // ── Asia ──
    GenericIdInfo { code: "AF", country_name: "Afghanistan", id_name: "Tazkira", length: 13, numeric_only: true },
    GenericIdInfo { code: "AM", country_name: "Armenia", id_name: "SSN", length: 10, numeric_only: true },
    GenericIdInfo { code: "AZ", country_name: "Azerbaijan", id_name: "FIN", length: 7, numeric_only: false },
    GenericIdInfo { code: "BD", country_name: "Bangladesh", id_name: "NID", length: 10, numeric_only: true },
    GenericIdInfo { code: "BN", country_name: "Brunei", id_name: "National ID", length: 9, numeric_only: false },
    GenericIdInfo { code: "BT", country_name: "Bhutan", id_name: "CID", length: 11, numeric_only: true },
    GenericIdInfo { code: "GE", country_name: "Georgia", id_name: "Personal Number", length: 11, numeric_only: true },
    GenericIdInfo { code: "IQ", country_name: "Iraq", id_name: "National ID", length: 12, numeric_only: true },
    GenericIdInfo { code: "IR", country_name: "Iran", id_name: "National Code", length: 10, numeric_only: true },
    GenericIdInfo { code: "JO", country_name: "Jordan", id_name: "National Number", length: 10, numeric_only: true },
    GenericIdInfo { code: "KG", country_name: "Kyrgyzstan", id_name: "PIN", length: 14, numeric_only: true },
    GenericIdInfo { code: "KH", country_name: "Cambodia", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "KP", country_name: "North Korea", id_name: "Citizen ID", length: 12, numeric_only: true },
    GenericIdInfo { code: "KW", country_name: "Kuwait", id_name: "Civil ID", length: 12, numeric_only: true },
    GenericIdInfo { code: "KZ", country_name: "Kazakhstan", id_name: "IIN", length: 12, numeric_only: true },
    GenericIdInfo { code: "LA", country_name: "Laos", id_name: "National ID", length: 13, numeric_only: true },
    GenericIdInfo { code: "LB", country_name: "Lebanon", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "LK", country_name: "Sri Lanka", id_name: "NIC", length: 12, numeric_only: true },
    GenericIdInfo { code: "MM", country_name: "Myanmar", id_name: "NRC", length: 12, numeric_only: false },
    GenericIdInfo { code: "MN", country_name: "Mongolia", id_name: "Register Number", length: 10, numeric_only: false },
    GenericIdInfo { code: "MO", country_name: "Macau", id_name: "BIR", length: 8, numeric_only: true },
    GenericIdInfo { code: "MV", country_name: "Maldives", id_name: "National ID", length: 7, numeric_only: false },
    GenericIdInfo { code: "NP", country_name: "Nepal", id_name: "Citizenship Number", length: 10, numeric_only: true },
    GenericIdInfo { code: "OM", country_name: "Oman", id_name: "Civil ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "PH", country_name: "Philippines", id_name: "PSN", length: 12, numeric_only: true },
    GenericIdInfo { code: "PK", country_name: "Pakistan", id_name: "CNIC", length: 13, numeric_only: true },
    GenericIdInfo { code: "QA", country_name: "Qatar", id_name: "QID", length: 11, numeric_only: true },
    GenericIdInfo { code: "SA", country_name: "Saudi Arabia", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "SY", country_name: "Syria", id_name: "National ID", length: 11, numeric_only: true },
    GenericIdInfo { code: "TJ", country_name: "Tajikistan", id_name: "PIN", length: 14, numeric_only: true },
    GenericIdInfo { code: "TM", country_name: "Turkmenistan", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "UZ", country_name: "Uzbekistan", id_name: "PINFL", length: 14, numeric_only: true },
    GenericIdInfo { code: "TL", country_name: "Timor-Leste", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "VN", country_name: "Vietnam", id_name: "CCCD", length: 12, numeric_only: true },
    GenericIdInfo { code: "YE", country_name: "Yemen", id_name: "National ID", length: 11, numeric_only: true },
    // ── Middle East ──
    GenericIdInfo { code: "AE", country_name: "United Arab Emirates", id_name: "Emirates ID", length: 15, numeric_only: true },
    GenericIdInfo { code: "BH", country_name: "Bahrain", id_name: "CPR", length: 9, numeric_only: true },
    GenericIdInfo { code: "PS", country_name: "Palestine", id_name: "National ID", length: 9, numeric_only: true },
    // ── Europe (not already specific) ──
    GenericIdInfo { code: "AX", country_name: "Aland Islands", id_name: "Henkilotunnus", length: 11, numeric_only: false },
    GenericIdInfo { code: "AD", country_name: "Andorra", id_name: "NRT", length: 8, numeric_only: false },
    GenericIdInfo { code: "AL", country_name: "Albania", id_name: "NID", length: 10, numeric_only: false },
    GenericIdInfo { code: "BY", country_name: "Belarus", id_name: "Personal Number", length: 14, numeric_only: false },
    GenericIdInfo { code: "CY", country_name: "Cyprus", id_name: "ARC", length: 8, numeric_only: true },
    GenericIdInfo { code: "GI", country_name: "Gibraltar", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "GG", country_name: "Guernsey", id_name: "SSN", length: 9, numeric_only: true },
    GenericIdInfo { code: "HU", country_name: "Hungary", id_name: "Personal ID", length: 11, numeric_only: true },
    GenericIdInfo { code: "IM", country_name: "Isle of Man", id_name: "National Insurance", length: 9, numeric_only: false },
    GenericIdInfo { code: "JE", country_name: "Jersey", id_name: "SSN", length: 9, numeric_only: true },
    GenericIdInfo { code: "LI", country_name: "Liechtenstein", id_name: "PEID", length: 12, numeric_only: true },
    GenericIdInfo { code: "LU", country_name: "Luxembourg", id_name: "National ID", length: 13, numeric_only: true },
    GenericIdInfo { code: "MC", country_name: "Monaco", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "MD", country_name: "Moldova", id_name: "IDNP", length: 13, numeric_only: true },
    GenericIdInfo { code: "MK", country_name: "North Macedonia", id_name: "EMBG", length: 13, numeric_only: true },
    GenericIdInfo { code: "MT", country_name: "Malta", id_name: "ID Card Number", length: 8, numeric_only: false },
    GenericIdInfo { code: "SM", country_name: "San Marino", id_name: "COE", length: 8, numeric_only: true },
    GenericIdInfo { code: "RU", country_name: "Russia", id_name: "Passport", length: 10, numeric_only: true },
    GenericIdInfo { code: "UA", country_name: "Ukraine", id_name: "RNTRC", length: 10, numeric_only: true },
    GenericIdInfo { code: "VA", country_name: "Vatican City", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "XK", country_name: "Kosovo", id_name: "Personal Number", length: 10, numeric_only: true },
    // ── Americas ──
    GenericIdInfo { code: "CR", country_name: "Costa Rica", id_name: "Cedula", length: 9, numeric_only: true },
    GenericIdInfo { code: "AG", country_name: "Antigua and Barbuda", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "AI", country_name: "Anguilla", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "AW", country_name: "Aruba", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "BB", country_name: "Barbados", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "BM", country_name: "Bermuda", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "BO", country_name: "Bolivia", id_name: "CI", length: 7, numeric_only: true },
    GenericIdInfo { code: "BQ", country_name: "Caribbean Netherlands", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "BS", country_name: "Bahamas", id_name: "NIB", length: 9, numeric_only: true },
    GenericIdInfo { code: "BZ", country_name: "Belize", id_name: "Social Security", length: 9, numeric_only: true },
    GenericIdInfo { code: "CU", country_name: "Cuba", id_name: "CI", length: 11, numeric_only: true },
    GenericIdInfo { code: "CW", country_name: "Curacao", id_name: "Sedula", length: 10, numeric_only: true },
    GenericIdInfo { code: "DM", country_name: "Dominica", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "DO", country_name: "Dominican Republic", id_name: "Cedula", length: 11, numeric_only: true },
    GenericIdInfo { code: "GD", country_name: "Grenada", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "GT", country_name: "Guatemala", id_name: "DPI", length: 13, numeric_only: true },
    GenericIdInfo { code: "GY", country_name: "Guyana", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "HN", country_name: "Honduras", id_name: "DNI", length: 13, numeric_only: true },
    GenericIdInfo { code: "HT", country_name: "Haiti", id_name: "CIN", length: 10, numeric_only: true },
    GenericIdInfo { code: "JM", country_name: "Jamaica", id_name: "TRN", length: 9, numeric_only: true },
    GenericIdInfo { code: "KN", country_name: "Saint Kitts and Nevis", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "KY", country_name: "Cayman Islands", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "LC", country_name: "Saint Lucia", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "MS", country_name: "Montserrat", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "NI", country_name: "Nicaragua", id_name: "Cedula", length: 14, numeric_only: false },
    GenericIdInfo { code: "PA", country_name: "Panama", id_name: "Cedula", length: 10, numeric_only: true },
    GenericIdInfo { code: "PY", country_name: "Paraguay", id_name: "CI", length: 8, numeric_only: true },
    GenericIdInfo { code: "SR", country_name: "Suriname", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "SV", country_name: "El Salvador", id_name: "DUI", length: 9, numeric_only: true },
    GenericIdInfo { code: "SX", country_name: "Sint Maarten", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "TC", country_name: "Turks and Caicos", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "TT", country_name: "Trinidad and Tobago", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "VC", country_name: "Saint Vincent", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "VE", country_name: "Venezuela", id_name: "CI", length: 8, numeric_only: true },
    GenericIdInfo { code: "FK", country_name: "Falkland Islands", id_name: "National ID", length: 8, numeric_only: true },
    // ── Africa ──
    GenericIdInfo { code: "AO", country_name: "Angola", id_name: "BI", length: 14, numeric_only: true },
    GenericIdInfo { code: "BF", country_name: "Burkina Faso", id_name: "CNIB", length: 11, numeric_only: false },
    GenericIdInfo { code: "BI", country_name: "Burundi", id_name: "CNI", length: 10, numeric_only: true },
    GenericIdInfo { code: "BJ", country_name: "Benin", id_name: "CIN", length: 10, numeric_only: true },
    GenericIdInfo { code: "BW", country_name: "Botswana", id_name: "Omang", length: 9, numeric_only: true },
    GenericIdInfo { code: "CD", country_name: "DR Congo", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "CF", country_name: "Central African Republic", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "CG", country_name: "Republic of Congo", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "CI", country_name: "Cote d'Ivoire", id_name: "CNI", length: 11, numeric_only: true },
    GenericIdInfo { code: "CM", country_name: "Cameroon", id_name: "CNI", length: 12, numeric_only: true },
    GenericIdInfo { code: "CV", country_name: "Cape Verde", id_name: "CNI", length: 9, numeric_only: true },
    GenericIdInfo { code: "DJ", country_name: "Djibouti", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "DZ", country_name: "Algeria", id_name: "NIN", length: 18, numeric_only: true },
    GenericIdInfo { code: "ER", country_name: "Eritrea", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "ET", country_name: "Ethiopia", id_name: "Fayda ID", length: 12, numeric_only: true },
    GenericIdInfo { code: "GA", country_name: "Gabon", id_name: "CNI", length: 10, numeric_only: true },
    GenericIdInfo { code: "GH", country_name: "Ghana", id_name: "Ghana Card", length: 15, numeric_only: false },
    GenericIdInfo { code: "GM", country_name: "Gambia", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "GN", country_name: "Guinea", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "GQ", country_name: "Equatorial Guinea", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "GW", country_name: "Guinea-Bissau", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "KE", country_name: "Kenya", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "KM", country_name: "Comoros", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "LR", country_name: "Liberia", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "LS", country_name: "Lesotho", id_name: "National ID", length: 11, numeric_only: true },
    GenericIdInfo { code: "LY", country_name: "Libya", id_name: "National Number", length: 12, numeric_only: true },
    GenericIdInfo { code: "MA", country_name: "Morocco", id_name: "CNIE", length: 8, numeric_only: false },
    GenericIdInfo { code: "MG", country_name: "Madagascar", id_name: "CIN", length: 12, numeric_only: true },
    GenericIdInfo { code: "ML", country_name: "Mali", id_name: "NINA", length: 14, numeric_only: true },
    GenericIdInfo { code: "MR", country_name: "Mauritania", id_name: "NNI", length: 10, numeric_only: true },
    GenericIdInfo { code: "MU", country_name: "Mauritius", id_name: "NIC", length: 14, numeric_only: false },
    GenericIdInfo { code: "MW", country_name: "Malawi", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "MZ", country_name: "Mozambique", id_name: "BI", length: 12, numeric_only: true },
    GenericIdInfo { code: "NA", country_name: "Namibia", id_name: "National ID", length: 11, numeric_only: true },
    GenericIdInfo { code: "NE", country_name: "Niger", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "NG", country_name: "Nigeria", id_name: "NIN", length: 11, numeric_only: true },
    GenericIdInfo { code: "RW", country_name: "Rwanda", id_name: "NID", length: 16, numeric_only: true },
    GenericIdInfo { code: "SC", country_name: "Seychelles", id_name: "NIN", length: 8, numeric_only: true },
    GenericIdInfo { code: "SD", country_name: "Sudan", id_name: "National Number", length: 12, numeric_only: true },
    GenericIdInfo { code: "SL", country_name: "Sierra Leone", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "SN", country_name: "Senegal", id_name: "CNI", length: 13, numeric_only: true },
    GenericIdInfo { code: "SO", country_name: "Somalia", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "SS", country_name: "South Sudan", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "ST", country_name: "Sao Tome and Principe", id_name: "BI", length: 9, numeric_only: true },
    GenericIdInfo { code: "SZ", country_name: "Eswatini", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "TD", country_name: "Chad", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "TG", country_name: "Togo", id_name: "National ID", length: 10, numeric_only: true },
    GenericIdInfo { code: "TN", country_name: "Tunisia", id_name: "CIN", length: 8, numeric_only: true },
    GenericIdInfo { code: "TZ", country_name: "Tanzania", id_name: "NIDA", length: 20, numeric_only: true },
    GenericIdInfo { code: "UG", country_name: "Uganda", id_name: "NIN", length: 14, numeric_only: false },
    GenericIdInfo { code: "ZM", country_name: "Zambia", id_name: "NRC", length: 9, numeric_only: true },
    GenericIdInfo { code: "ZW", country_name: "Zimbabwe", id_name: "National ID", length: 11, numeric_only: false },
    // ── Oceania ──
    GenericIdInfo { code: "FJ", country_name: "Fiji", id_name: "TIN", length: 9, numeric_only: true },
    GenericIdInfo { code: "FM", country_name: "Micronesia", id_name: "SSN", length: 9, numeric_only: true },
    GenericIdInfo { code: "KI", country_name: "Kiribati", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "MH", country_name: "Marshall Islands", id_name: "SSN", length: 9, numeric_only: true },
    GenericIdInfo { code: "NR", country_name: "Nauru", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "PG", country_name: "Papua New Guinea", id_name: "NID", length: 11, numeric_only: true },
    GenericIdInfo { code: "PW", country_name: "Palau", id_name: "SSN", length: 9, numeric_only: true },
    GenericIdInfo { code: "SB", country_name: "Solomon Islands", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "TO", country_name: "Tonga", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "TV", country_name: "Tuvalu", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "VU", country_name: "Vanuatu", id_name: "National ID", length: 8, numeric_only: true },
    GenericIdInfo { code: "WS", country_name: "Samoa", id_name: "National ID", length: 8, numeric_only: true },
    // ── Special ──
    GenericIdInfo { code: "AQ", country_name: "Antarctica", id_name: "National ID", length: 9, numeric_only: true },
    GenericIdInfo { code: "EH", country_name: "Western Sahara", id_name: "CNIE", length: 8, numeric_only: false },
];

fn find_info(code: &str) -> Option<&'static GenericIdInfo> {
    GENERIC_COUNTRIES.iter().find(|c| c.code == code)
}

pub fn is_supported(code: &str) -> bool {
    find_info(code).is_some()
}

pub fn generate(code: &str, _opts: &super::GenOptions, rng: &mut impl Rng) -> Option<String> {
    let info = find_info(code)?;
    let id: String = if info.numeric_only {
        (0..info.length)
            .map(|i| {
                if i == 0 {
                    (b'0' + rng.gen_range(1..=9u8)) as char
                } else {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                }
            })
            .collect()
    } else {
        (0..info.length)
            .map(|i| {
                if i == 0 {
                    (b'A' + rng.gen_range(0..26u8)) as char
                } else if rng.gen_bool(0.5) {
                    (b'0' + rng.gen_range(0..=9u8)) as char
                } else {
                    (b'A' + rng.gen_range(0..26u8)) as char
                }
            })
            .collect()
    };
    Some(id)
}

pub fn validate(code: &str, id: &str) -> Option<bool> {
    let info = find_info(code)?;
    let ok = id.len() == info.length as usize
        && if info.numeric_only {
            id.chars().all(|c| c.is_ascii_digit())
        } else {
            id.chars().all(|c| c.is_ascii_alphanumeric())
        };
    Some(ok)
}

pub fn parse(code: &str, id: &str) -> Option<super::IdResult> {
    let valid = validate(code, id).unwrap_or(false);
    let info = find_info(code)?;
    let _ = info; // just to confirm it exists
    Some(super::IdResult {
        code: id.to_string(),
        gender: None,
        dob: None,
        valid,
    })
}

pub fn list_countries() -> Vec<(&'static str, &'static str, &'static str)> {
    GENERIC_COUNTRIES
        .iter()
        .map(|c| (c.code, c.country_name, c.id_name))
        .collect()
}

pub fn id_name(code: &str) -> Option<&'static str> {
    find_info(code).map(|info| info.id_name)
}

pub fn country_name(code: &str) -> Option<&'static str> {
    find_info(code).map(|info| info.country_name)
}
