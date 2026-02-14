//! Centralized registry of ISO 3166-1 alpha-2 country codes and names.
//! This list covers 250+ countries and territories supported by the library.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountryInfo {
    pub code: &'static str,
    pub name: &'static str,
}

/// A comprehensive list of all supported ISO 3166-1 alpha-2 country codes and names.
pub const ALL_COUNTRIES: &[CountryInfo] = &[
    CountryInfo {
        code: "AD",
        name: "Andorra",
    },
    CountryInfo {
        code: "AE",
        name: "United Arab Emirates",
    },
    CountryInfo {
        code: "AF",
        name: "Afghanistan",
    },
    CountryInfo {
        code: "AG",
        name: "Antigua and Barbuda",
    },
    CountryInfo {
        code: "AI",
        name: "Anguilla",
    },
    CountryInfo {
        code: "AL",
        name: "Albania",
    },
    CountryInfo {
        code: "AM",
        name: "Armenia",
    },
    CountryInfo {
        code: "AO",
        name: "Angola",
    },
    CountryInfo {
        code: "AQ",
        name: "Antarctica",
    },
    CountryInfo {
        code: "AR",
        name: "Argentina",
    },
    CountryInfo {
        code: "AS",
        name: "American Samoa",
    },
    CountryInfo {
        code: "AT",
        name: "Austria",
    },
    CountryInfo {
        code: "AU",
        name: "Australia",
    },
    CountryInfo {
        code: "AW",
        name: "Aruba",
    },
    CountryInfo {
        code: "AX",
        name: "Aland Islands",
    },
    CountryInfo {
        code: "AZ",
        name: "Azerbaijan",
    },
    CountryInfo {
        code: "BA",
        name: "Bosnia and Herzegovina",
    },
    CountryInfo {
        code: "BB",
        name: "Barbados",
    },
    CountryInfo {
        code: "BD",
        name: "Bangladesh",
    },
    CountryInfo {
        code: "BE",
        name: "Belgium",
    },
    CountryInfo {
        code: "BF",
        name: "Burkina Faso",
    },
    CountryInfo {
        code: "BG",
        name: "Bulgaria",
    },
    CountryInfo {
        code: "BH",
        name: "Bahrain",
    },
    CountryInfo {
        code: "BI",
        name: "Burundi",
    },
    CountryInfo {
        code: "BJ",
        name: "Benin",
    },
    CountryInfo {
        code: "BM",
        name: "Bermuda",
    },
    CountryInfo {
        code: "BN",
        name: "Brunei",
    },
    CountryInfo {
        code: "BO",
        name: "Bolivia",
    },
    CountryInfo {
        code: "BQ",
        name: "Caribbean Netherlands",
    },
    CountryInfo {
        code: "BR",
        name: "Brazil",
    },
    CountryInfo {
        code: "BS",
        name: "Bahamas",
    },
    CountryInfo {
        code: "BT",
        name: "Bhutan",
    },
    CountryInfo {
        code: "BW",
        name: "Botswana",
    },
    CountryInfo {
        code: "BY",
        name: "Belarus",
    },
    CountryInfo {
        code: "BZ",
        name: "Belize",
    },
    CountryInfo {
        code: "CA",
        name: "Canada",
    },
    CountryInfo {
        code: "CD",
        name: "DR Congo",
    },
    CountryInfo {
        code: "CF",
        name: "Central African Republic",
    },
    CountryInfo {
        code: "CG",
        name: "Republic of Congo",
    },
    CountryInfo {
        code: "CH",
        name: "Switzerland",
    },
    CountryInfo {
        code: "CI",
        name: "Cote d'Ivoire",
    },
    CountryInfo {
        code: "CL",
        name: "Chile",
    },
    CountryInfo {
        code: "CM",
        name: "Cameroon",
    },
    CountryInfo {
        code: "CN",
        name: "China",
    },
    CountryInfo {
        code: "CO",
        name: "Colombia",
    },
    CountryInfo {
        code: "CR",
        name: "Costa Rica",
    },
    CountryInfo {
        code: "CU",
        name: "Cuba",
    },
    CountryInfo {
        code: "CV",
        name: "Cape Verde",
    },
    CountryInfo {
        code: "CW",
        name: "Curacao",
    },
    CountryInfo {
        code: "CY",
        name: "Cyprus",
    },
    CountryInfo {
        code: "CZ",
        name: "Czech Republic",
    },
    CountryInfo {
        code: "DE",
        name: "Germany",
    },
    CountryInfo {
        code: "DJ",
        name: "Djibouti",
    },
    CountryInfo {
        code: "DK",
        name: "Denmark",
    },
    CountryInfo {
        code: "DM",
        name: "Dominica",
    },
    CountryInfo {
        code: "DO",
        name: "Dominican Republic",
    },
    CountryInfo {
        code: "DZ",
        name: "Algeria",
    },
    CountryInfo {
        code: "EC",
        name: "Ecuador",
    },
    CountryInfo {
        code: "EE",
        name: "Estonia",
    },
    CountryInfo {
        code: "EG",
        name: "Egypt",
    },
    CountryInfo {
        code: "EH",
        name: "Western Sahara",
    },
    CountryInfo {
        code: "ER",
        name: "Eritrea",
    },
    CountryInfo {
        code: "ES",
        name: "Spain",
    },
    CountryInfo {
        code: "ET",
        name: "Ethiopia",
    },
    CountryInfo {
        code: "FI",
        name: "Finland",
    },
    CountryInfo {
        code: "FJ",
        name: "Fiji",
    },
    CountryInfo {
        code: "FK",
        name: "Falkland Islands",
    },
    CountryInfo {
        code: "FM",
        name: "Micronesia",
    },
    CountryInfo {
        code: "FO",
        name: "Country FO",
    },
    CountryInfo {
        code: "FR",
        name: "France",
    },
    CountryInfo {
        code: "GA",
        name: "Gabon",
    },
    CountryInfo {
        code: "GB",
        name: "United Kingdom",
    },
    CountryInfo {
        code: "GD",
        name: "Grenada",
    },
    CountryInfo {
        code: "GE",
        name: "Georgia",
    },
    CountryInfo {
        code: "GF",
        name: "Country GF",
    },
    CountryInfo {
        code: "GG",
        name: "Guernsey",
    },
    CountryInfo {
        code: "GH",
        name: "Ghana",
    },
    CountryInfo {
        code: "GI",
        name: "Gibraltar",
    },
    CountryInfo {
        code: "GL",
        name: "Country GL",
    },
    CountryInfo {
        code: "GM",
        name: "Gambia",
    },
    CountryInfo {
        code: "GN",
        name: "Guinea",
    },
    CountryInfo {
        code: "GP",
        name: "Country GP",
    },
    CountryInfo {
        code: "GQ",
        name: "Equatorial Guinea",
    },
    CountryInfo {
        code: "GR",
        name: "Greece",
    },
    CountryInfo {
        code: "GT",
        name: "Guatemala",
    },
    CountryInfo {
        code: "GW",
        name: "Guinea-Bissau",
    },
    CountryInfo {
        code: "GY",
        name: "Guyana",
    },
    CountryInfo {
        code: "HK",
        name: "Hong Kong",
    },
    CountryInfo {
        code: "HN",
        name: "Honduras",
    },
    CountryInfo {
        code: "HR",
        name: "Croatia",
    },
    CountryInfo {
        code: "HT",
        name: "Haiti",
    },
    CountryInfo {
        code: "HU",
        name: "Hungary",
    },
    CountryInfo {
        code: "ID",
        name: "Indonesia",
    },
    CountryInfo {
        code: "IE",
        name: "Ireland",
    },
    CountryInfo {
        code: "IL",
        name: "Israel",
    },
    CountryInfo {
        code: "IM",
        name: "Isle of Man",
    },
    CountryInfo {
        code: "IN",
        name: "India",
    },
    CountryInfo {
        code: "IQ",
        name: "Iraq",
    },
    CountryInfo {
        code: "IR",
        name: "Iran",
    },
    CountryInfo {
        code: "IS",
        name: "Iceland",
    },
    CountryInfo {
        code: "IT",
        name: "Italy",
    },
    CountryInfo {
        code: "JE",
        name: "Jersey",
    },
    CountryInfo {
        code: "JM",
        name: "Jamaica",
    },
    CountryInfo {
        code: "JO",
        name: "Jordan",
    },
    CountryInfo {
        code: "JP",
        name: "Japan",
    },
    CountryInfo {
        code: "KE",
        name: "Kenya",
    },
    CountryInfo {
        code: "KG",
        name: "Kyrgyzstan",
    },
    CountryInfo {
        code: "KH",
        name: "Cambodia",
    },
    CountryInfo {
        code: "KI",
        name: "Kiribati",
    },
    CountryInfo {
        code: "KM",
        name: "Comoros",
    },
    CountryInfo {
        code: "KN",
        name: "Saint Kitts and Nevis",
    },
    CountryInfo {
        code: "KP",
        name: "North Korea",
    },
    CountryInfo {
        code: "KR",
        name: "South Korea",
    },
    CountryInfo {
        code: "KW",
        name: "Kuwait",
    },
    CountryInfo {
        code: "KY",
        name: "Cayman Islands",
    },
    CountryInfo {
        code: "KZ",
        name: "Kazakhstan",
    },
    CountryInfo {
        code: "LA",
        name: "Laos",
    },
    CountryInfo {
        code: "LB",
        name: "Lebanon",
    },
    CountryInfo {
        code: "LC",
        name: "Saint Lucia",
    },
    CountryInfo {
        code: "LI",
        name: "Liechtenstein",
    },
    CountryInfo {
        code: "LK",
        name: "Sri Lanka",
    },
    CountryInfo {
        code: "LR",
        name: "Liberia",
    },
    CountryInfo {
        code: "LS",
        name: "Lesotho",
    },
    CountryInfo {
        code: "LT",
        name: "Lithuania",
    },
    CountryInfo {
        code: "LU",
        name: "Luxembourg",
    },
    CountryInfo {
        code: "LV",
        name: "Latvia",
    },
    CountryInfo {
        code: "LY",
        name: "Libya",
    },
    CountryInfo {
        code: "MA",
        name: "Morocco",
    },
    CountryInfo {
        code: "MC",
        name: "Monaco",
    },
    CountryInfo {
        code: "MD",
        name: "Moldova",
    },
    CountryInfo {
        code: "ME",
        name: "Montenegro",
    },
    CountryInfo {
        code: "MF",
        name: "Country MF",
    },
    CountryInfo {
        code: "MG",
        name: "Madagascar",
    },
    CountryInfo {
        code: "MH",
        name: "Marshall Islands",
    },
    CountryInfo {
        code: "MK",
        name: "North Macedonia",
    },
    CountryInfo {
        code: "ML",
        name: "Mali",
    },
    CountryInfo {
        code: "MM",
        name: "Myanmar",
    },
    CountryInfo {
        code: "MN",
        name: "Mongolia",
    },
    CountryInfo {
        code: "MO",
        name: "Macau",
    },
    CountryInfo {
        code: "MQ",
        name: "Country MQ",
    },
    CountryInfo {
        code: "MR",
        name: "Mauritania",
    },
    CountryInfo {
        code: "MS",
        name: "Montserrat",
    },
    CountryInfo {
        code: "MT",
        name: "Malta",
    },
    CountryInfo {
        code: "MU",
        name: "Mauritius",
    },
    CountryInfo {
        code: "MV",
        name: "Maldives",
    },
    CountryInfo {
        code: "MW",
        name: "Malawi",
    },
    CountryInfo {
        code: "MX",
        name: "Mexico",
    },
    CountryInfo {
        code: "MY",
        name: "Malaysia",
    },
    CountryInfo {
        code: "MZ",
        name: "Mozambique",
    },
    CountryInfo {
        code: "NA",
        name: "Namibia",
    },
    CountryInfo {
        code: "NC",
        name: "Country NC",
    },
    CountryInfo {
        code: "NE",
        name: "Niger",
    },
    CountryInfo {
        code: "NG",
        name: "Nigeria",
    },
    CountryInfo {
        code: "NI",
        name: "Nicaragua",
    },
    CountryInfo {
        code: "NL",
        name: "Netherlands",
    },
    CountryInfo {
        code: "NO",
        name: "Norway",
    },
    CountryInfo {
        code: "NP",
        name: "Nepal",
    },
    CountryInfo {
        code: "NR",
        name: "Nauru",
    },
    CountryInfo {
        code: "NZ",
        name: "New Zealand",
    },
    CountryInfo {
        code: "OM",
        name: "Oman",
    },
    CountryInfo {
        code: "PA",
        name: "Panama",
    },
    CountryInfo {
        code: "PE",
        name: "Peru",
    },
    CountryInfo {
        code: "PF",
        name: "Country PF",
    },
    CountryInfo {
        code: "PG",
        name: "Papua New Guinea",
    },
    CountryInfo {
        code: "PH",
        name: "Philippines",
    },
    CountryInfo {
        code: "PK",
        name: "Pakistan",
    },
    CountryInfo {
        code: "PL",
        name: "Poland",
    },
    CountryInfo {
        code: "PM",
        name: "Country PM",
    },
    CountryInfo {
        code: "PS",
        name: "Palestine",
    },
    CountryInfo {
        code: "PT",
        name: "Portugal",
    },
    CountryInfo {
        code: "PW",
        name: "Palau",
    },
    CountryInfo {
        code: "PY",
        name: "Paraguay",
    },
    CountryInfo {
        code: "QA",
        name: "Qatar",
    },
    CountryInfo {
        code: "RE",
        name: "Country RE",
    },
    CountryInfo {
        code: "RO",
        name: "Romania",
    },
    CountryInfo {
        code: "RS",
        name: "Serbia",
    },
    CountryInfo {
        code: "RU",
        name: "Russia",
    },
    CountryInfo {
        code: "RW",
        name: "Rwanda",
    },
    CountryInfo {
        code: "SA",
        name: "Saudi Arabia",
    },
    CountryInfo {
        code: "SB",
        name: "Solomon Islands",
    },
    CountryInfo {
        code: "SC",
        name: "Seychelles",
    },
    CountryInfo {
        code: "SD",
        name: "Sudan",
    },
    CountryInfo {
        code: "SE",
        name: "Sweden",
    },
    CountryInfo {
        code: "SG",
        name: "Singapore",
    },
    CountryInfo {
        code: "SI",
        name: "Slovenia",
    },
    CountryInfo {
        code: "SK",
        name: "Slovakia",
    },
    CountryInfo {
        code: "SL",
        name: "Sierra Leone",
    },
    CountryInfo {
        code: "SM",
        name: "San Marino",
    },
    CountryInfo {
        code: "SN",
        name: "Senegal",
    },
    CountryInfo {
        code: "SO",
        name: "Somalia",
    },
    CountryInfo {
        code: "SR",
        name: "Suriname",
    },
    CountryInfo {
        code: "SS",
        name: "South Sudan",
    },
    CountryInfo {
        code: "ST",
        name: "Sao Tome and Principe",
    },
    CountryInfo {
        code: "SV",
        name: "El Salvador",
    },
    CountryInfo {
        code: "SX",
        name: "Sint Maarten",
    },
    CountryInfo {
        code: "SY",
        name: "Syria",
    },
    CountryInfo {
        code: "SZ",
        name: "Eswatini",
    },
    CountryInfo {
        code: "TC",
        name: "Turks and Caicos",
    },
    CountryInfo {
        code: "TD",
        name: "Chad",
    },
    CountryInfo {
        code: "TF",
        name: "Country TF",
    },
    CountryInfo {
        code: "TG",
        name: "Togo",
    },
    CountryInfo {
        code: "TH",
        name: "Thailand",
    },
    CountryInfo {
        code: "TJ",
        name: "Tajikistan",
    },
    CountryInfo {
        code: "TL",
        name: "Timor-Leste",
    },
    CountryInfo {
        code: "TM",
        name: "Turkmenistan",
    },
    CountryInfo {
        code: "TN",
        name: "Tunisia",
    },
    CountryInfo {
        code: "TO",
        name: "Tonga",
    },
    CountryInfo {
        code: "TR",
        name: "Turkey",
    },
    CountryInfo {
        code: "TT",
        name: "Trinidad and Tobago",
    },
    CountryInfo {
        code: "TV",
        name: "Tuvalu",
    },
    CountryInfo {
        code: "TW",
        name: "Taiwan",
    },
    CountryInfo {
        code: "TZ",
        name: "Tanzania",
    },
    CountryInfo {
        code: "UA",
        name: "Ukraine",
    },
    CountryInfo {
        code: "UG",
        name: "Uganda",
    },
    CountryInfo {
        code: "US",
        name: "United States",
    },
    CountryInfo {
        code: "UY",
        name: "Uruguay",
    },
    CountryInfo {
        code: "UZ",
        name: "Uzbekistan",
    },
    CountryInfo {
        code: "VA",
        name: "Vatican City",
    },
    CountryInfo {
        code: "VC",
        name: "Saint Vincent",
    },
    CountryInfo {
        code: "VE",
        name: "Venezuela",
    },
    CountryInfo {
        code: "VG",
        name: "Country VG",
    },
    CountryInfo {
        code: "VN",
        name: "Vietnam",
    },
    CountryInfo {
        code: "VU",
        name: "Vanuatu",
    },
    CountryInfo {
        code: "WF",
        name: "Country WF",
    },
    CountryInfo {
        code: "WS",
        name: "Samoa",
    },
    CountryInfo {
        code: "XK",
        name: "Kosovo",
    },
    CountryInfo {
        code: "YE",
        name: "Yemen",
    },
    CountryInfo {
        code: "YT",
        name: "Country YT",
    },
    CountryInfo {
        code: "ZA",
        name: "South Africa",
    },
    CountryInfo {
        code: "ZM",
        name: "Zambia",
    },
    CountryInfo {
        code: "ZW",
        name: "Zimbabwe",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerritoryAlias {
    pub code: &'static str,
    pub name: &'static str,
    pub parent_code: &'static str,
}

/// Maps territories and dependencies to their parent sovereign state for identifier resolution.
pub const TERRITORY_ALIASES: &[TerritoryAlias] = &[
    TerritoryAlias {
        code: "AS",
        name: "American Samoa",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "BL",
        name: "Saint Barthelemy",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "BV",
        name: "Bouvet Island",
        parent_code: "NO",
    },
    TerritoryAlias {
        code: "CC",
        name: "Cocos (Keeling) Islands",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "CK",
        name: "Cook Islands",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "CX",
        name: "Christmas Island",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "FO",
        name: "Faroe Islands",
        parent_code: "DK",
    },
    TerritoryAlias {
        code: "GF",
        name: "French Guiana",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "GL",
        name: "Greenland",
        parent_code: "DK",
    },
    TerritoryAlias {
        code: "GP",
        name: "Guadeloupe",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "GS",
        name: "South Georgia and the South Sandwich Islands",
        parent_code: "GB",
    },
    TerritoryAlias {
        code: "GU",
        name: "Guam",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "HM",
        name: "Heard Island and McDonald Islands",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "IO",
        name: "British Indian Ocean Territory",
        parent_code: "GB",
    },
    TerritoryAlias {
        code: "MF",
        name: "Saint Martin",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "MP",
        name: "Northern Mariana Islands",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "MQ",
        name: "Martinique",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "NC",
        name: "New Caledonia",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "NF",
        name: "Norfolk Island",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "NU",
        name: "Niue",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "PF",
        name: "French Polynesia",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "PM",
        name: "Saint Pierre and Miquelon",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "PN",
        name: "Pitcairn Islands",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "PR",
        name: "Puerto Rico",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "RE",
        name: "Reunion",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "SH",
        name: "Saint Helena, Ascension and Tristan da Cunha",
        parent_code: "GB",
    },
    TerritoryAlias {
        code: "SJ",
        name: "Svalbard and Jan Mayen",
        parent_code: "NO",
    },
    TerritoryAlias {
        code: "TF",
        name: "French Southern Territories",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "TK",
        name: "Tokelau",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "UM",
        name: "US Minor Outlying Islands",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "VG",
        name: "British Virgin Islands",
        parent_code: "GB",
    },
    TerritoryAlias {
        code: "VI",
        name: "US Virgin Islands",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "WF",
        name: "Wallis and Futuna",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "YT",
        name: "Mayotte",
        parent_code: "FR",
    },
];

/// Returns the name of a country by its ISO code, or None if not found.
pub fn get_country_name(code: &str) -> Option<&'static str> {
    let code = code.to_uppercase();
    ALL_COUNTRIES
        .iter()
        .find(|c| c.code == code)
        .map(|c| c.name)
        .or_else(|| {
            TERRITORY_ALIASES
                .iter()
                .find(|a| a.code == code)
                .map(|a| a.name)
        })
}

/// Returns the parent country code for a territory, if applicable.
pub fn resolve_territory(code: &str) -> Option<&'static str> {
    let code = code.to_uppercase();
    TERRITORY_ALIASES
        .iter()
        .find(|a| a.code == code)
        .map(|a| a.parent_code)
}

/// Returns true if the ISO code is supported.
pub fn is_supported(code: &str) -> bool {
    let code = code.to_uppercase();
    ALL_COUNTRIES.iter().any(|c| c.code == code) || TERRITORY_ALIASES.iter().any(|a| a.code == code)
}

/// Returns all supported ISO 3166-1 alpha-2 codes.
pub fn all_codes() -> Vec<&'static str> {
    let mut codes: Vec<_> = ALL_COUNTRIES.iter().map(|c| c.code).collect();
    for a in TERRITORY_ALIASES {
        codes.push(a.code);
    }
    codes.sort();
    codes
}
