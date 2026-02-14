pub mod checksum;
pub mod generic;
pub mod iban_based;

pub mod ar;
pub mod au;
pub mod br;
pub mod ca;
pub mod cn;
pub mod gb;
pub mod hk;
pub mod in_;
pub mod jp;
pub mod kr;
pub mod mx;
pub mod ng;
pub mod nz;
pub mod sg;
pub mod us;
pub mod za;

/// Options for generating a bank account.
#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub bank_code: Option<String>,
}

/// Result of generating or validating a bank account number.
#[derive(Debug, Clone)]
pub struct AccountResult {
    pub country_code: String,
    pub country_name: String,
    pub format_name: String,
    pub bank_code: Option<String>,
    pub branch_code: Option<String>,
    pub account_number: String,
    pub check_digits: Option<String>,
    pub formatted: String,
    pub raw: String,
    pub iban: Option<String>,
    pub valid: bool,
}

type GenerateFn = fn(&GenOptions, &mut rand::rngs::ThreadRng) -> AccountResult;
type ValidateFn = fn(&str) -> bool;
type FormatFn = fn(&str) -> String;

struct RegistryEntry {
    code: &'static str,
    country_name: &'static str,
    format_name: &'static str,
    has_iban: bool,
    generate: GenerateFn,
    validate: ValidateFn,
    format: FormatFn,
}

struct TerritoryAlias {
    code: &'static str,
    country_name: &'static str,
    parent_code: &'static str,
}

/// Territories that use a parent country's banking system.
static TERRITORY_ALIASES: &[TerritoryAlias] = &[
    // US territories
    TerritoryAlias { code: "AS", country_name: "American Samoa", parent_code: "US" },
    TerritoryAlias { code: "GU", country_name: "Guam", parent_code: "US" },
    TerritoryAlias { code: "MP", country_name: "Northern Mariana Islands", parent_code: "US" },
    TerritoryAlias { code: "PR", country_name: "Puerto Rico", parent_code: "US" },
    TerritoryAlias { code: "UM", country_name: "US Minor Outlying Islands", parent_code: "US" },
    TerritoryAlias { code: "VI", country_name: "US Virgin Islands", parent_code: "US" },
    // Australian territories
    TerritoryAlias { code: "CC", country_name: "Cocos (Keeling) Islands", parent_code: "AU" },
    TerritoryAlias { code: "CX", country_name: "Christmas Island", parent_code: "AU" },
    TerritoryAlias { code: "HM", country_name: "Heard Island and McDonald Islands", parent_code: "AU" },
    TerritoryAlias { code: "NF", country_name: "Norfolk Island", parent_code: "AU" },
    // NZ territories
    TerritoryAlias { code: "CK", country_name: "Cook Islands", parent_code: "NZ" },
    TerritoryAlias { code: "NU", country_name: "Niue", parent_code: "NZ" },
    TerritoryAlias { code: "PN", country_name: "Pitcairn Islands", parent_code: "NZ" },
    TerritoryAlias { code: "TK", country_name: "Tokelau", parent_code: "NZ" },
    // French territory
    TerritoryAlias { code: "BL", country_name: "Saint Barthélemy", parent_code: "FR" },
    // UK territories
    TerritoryAlias { code: "GS", country_name: "South Georgia and the South Sandwich Islands", parent_code: "GB" },
    TerritoryAlias { code: "IO", country_name: "British Indian Ocean Territory", parent_code: "GB" },
    TerritoryAlias { code: "SH", country_name: "Saint Helena, Ascension and Tristan da Cunha", parent_code: "GB" },
    // Norwegian territories
    TerritoryAlias { code: "BV", country_name: "Bouvet Island", parent_code: "NO" },
    TerritoryAlias { code: "SJ", country_name: "Svalbard and Jan Mayen", parent_code: "NO" },
    // Morocco
    TerritoryAlias { code: "EH", country_name: "Western Sahara", parent_code: "MA" },
    // Antarctica — no single banking system; mapped to generic
    TerritoryAlias { code: "AQ", country_name: "Antarctica", parent_code: "AQ" },
];

fn resolve_alias(code: &str) -> Option<&'static TerritoryAlias> {
    TERRITORY_ALIASES.iter().find(|a| a.code == code)
}

pub struct Registry {
    entries: Vec<RegistryEntry>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let entries = vec![
            RegistryEntry {
                code: "US",
                country_name: "United States",
                format_name: "ABA Routing + Account",
                has_iban: false,
                generate: us::generate,
                validate: us::validate,
                format: us::format,
            },
            RegistryEntry {
                code: "CA",
                country_name: "Canada",
                format_name: "Inst + Transit + Account",
                has_iban: false,
                generate: ca::generate,
                validate: ca::validate,
                format: ca::format,
            },
            RegistryEntry {
                code: "MX",
                country_name: "Mexico",
                format_name: "CLABE",
                has_iban: false,
                generate: mx::generate,
                validate: mx::validate,
                format: mx::format,
            },
            RegistryEntry {
                code: "AU",
                country_name: "Australia",
                format_name: "BSB + Account",
                has_iban: false,
                generate: au::generate,
                validate: au::validate,
                format: au::format,
            },
            RegistryEntry {
                code: "IN",
                country_name: "India",
                format_name: "IFSC + Account",
                has_iban: false,
                generate: in_::generate,
                validate: in_::validate,
                format: in_::format,
            },
            RegistryEntry {
                code: "JP",
                country_name: "Japan",
                format_name: "Bank + Branch + Account",
                has_iban: false,
                generate: jp::generate,
                validate: jp::validate,
                format: jp::format,
            },
            RegistryEntry {
                code: "CN",
                country_name: "China",
                format_name: "Bank Account (Luhn)",
                has_iban: false,
                generate: cn::generate,
                validate: cn::validate,
                format: cn::format,
            },
            RegistryEntry {
                code: "ZA",
                country_name: "South Africa",
                format_name: "Branch + Account",
                has_iban: false,
                generate: za::generate,
                validate: za::validate,
                format: za::format,
            },
            RegistryEntry {
                code: "NZ",
                country_name: "New Zealand",
                format_name: "Bank + Branch + Account + Suffix",
                has_iban: false,
                generate: nz::generate,
                validate: nz::validate,
                format: nz::format,
            },
            RegistryEntry {
                code: "SG",
                country_name: "Singapore",
                format_name: "Bank + Branch + Account",
                has_iban: false,
                generate: sg::generate,
                validate: sg::validate,
                format: sg::format,
            },
            RegistryEntry {
                code: "HK",
                country_name: "Hong Kong",
                format_name: "Bank + Account",
                has_iban: false,
                generate: hk::generate,
                validate: hk::validate,
                format: hk::format,
            },
            RegistryEntry {
                code: "KR",
                country_name: "South Korea",
                format_name: "Bank Account",
                has_iban: false,
                generate: kr::generate,
                validate: kr::validate,
                format: kr::format,
            },
            RegistryEntry {
                code: "BR",
                country_name: "Brazil",
                format_name: "Bank + Branch + Account",
                has_iban: false,
                generate: br::generate,
                validate: br::validate,
                format: br::format,
            },
            RegistryEntry {
                code: "GB",
                country_name: "United Kingdom",
                format_name: "Sort Code + Account",
                has_iban: true,
                generate: gb::generate,
                validate: gb::validate,
                format: gb::format,
            },
            RegistryEntry {
                code: "AR",
                country_name: "Argentina",
                format_name: "CBU",
                has_iban: false,
                generate: ar::generate,
                validate: ar::validate,
                format: ar::format,
            },
            RegistryEntry {
                code: "NG",
                country_name: "Nigeria",
                format_name: "NUBAN",
                has_iban: false,
                generate: ng::generate,
                validate: ng::validate,
                format: ng::format,
            },
        ];
        Registry { entries }
    }

    fn find(&self, country: &str) -> Option<&RegistryEntry> {
        self.entries.iter().find(|e| e.code == country)
    }

    pub fn is_supported(&self, country: &str) -> bool {
        self.find(country).is_some()
            || iban_based::is_supported(country)
            || generic::is_supported(country)
            || resolve_alias(country).is_some()
    }

    fn generate_for(&self, code: &str, opts: &GenOptions, rng: &mut rand::rngs::ThreadRng) -> Option<AccountResult> {
        if let Some(entry) = self.find(code) {
            let mut result = (entry.generate)(opts, rng);
            if entry.has_iban {
                if let Ok(iban) = crate::iban::generate_iban(Some(code), rng) {
                    result.iban = Some(iban);
                }
            }
            return Some(result);
        }
        if let Some(result) = iban_based::generate(code, rng) {
            return Some(result);
        }
        generic::generate(code, rng)
    }

    pub fn generate(
        &self,
        country: &str,
        opts: &GenOptions,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<AccountResult> {
        // Try direct lookup first
        if let Some(result) = self.generate_for(country, opts, rng) {
            return Some(result);
        }
        // Try alias
        if let Some(alias) = resolve_alias(country) {
            let mut result = self.generate_for(alias.parent_code, opts, rng)?;
            result.country_code = country.to_string();
            result.country_name = alias.country_name.to_string();
            return Some(result);
        }
        None
    }

    fn validate_for(&self, code: &str, raw: &str) -> Option<bool> {
        if let Some(entry) = self.find(code) {
            return Some((entry.validate)(raw));
        }
        if let Some(valid) = iban_based::validate(code, raw) {
            return Some(valid);
        }
        generic::validate(code, raw)
    }

    pub fn validate(&self, country: &str, raw: &str) -> Option<bool> {
        if let Some(result) = self.validate_for(country, raw) {
            return Some(result);
        }
        if let Some(alias) = resolve_alias(country) {
            return self.validate_for(alias.parent_code, raw);
        }
        None
    }

    fn format_for(&self, code: &str, raw: &str) -> Option<String> {
        if let Some(entry) = self.find(code) {
            return Some((entry.format)(raw));
        }
        if let Some(formatted) = iban_based::format(code, raw) {
            return Some(formatted);
        }
        generic::format(code, raw)
    }

    pub fn format(&self, country: &str, raw: &str) -> Option<String> {
        if let Some(result) = self.format_for(country, raw) {
            return Some(result);
        }
        if let Some(alias) = resolve_alias(country) {
            return self.format_for(alias.parent_code, raw);
        }
        None
    }

    pub fn list_countries(&self) -> Vec<(&str, &str, &str, bool)> {
        let mut seen = std::collections::HashSet::new();
        let mut result: Vec<(&str, &str, &str, bool)> = Vec::new();

        // Specific entries first
        for e in &self.entries {
            if seen.insert(e.code) {
                result.push((e.code, e.country_name, e.format_name, e.has_iban));
            }
        }

        // IBAN-based countries (skip those already in specific)
        for item in iban_based::list_countries() {
            if seen.insert(item.0) {
                result.push(item);
            }
        }

        // Generic countries
        for item in generic::list_countries() {
            if seen.insert(item.0) {
                result.push(item);
            }
        }

        // Territory aliases
        for alias in TERRITORY_ALIASES {
            if seen.insert(alias.code) {
                // Find the parent's format info
                let (format_name, has_iban) = if let Some(e) = self.entries.iter().find(|e| e.code == alias.parent_code) {
                    (e.format_name, e.has_iban)
                } else if iban_based::is_supported(alias.parent_code) {
                    ("IBAN Account", true)
                } else {
                    ("Bank + Account", false)
                };
                result.push((alias.code, alias.country_name, format_name, has_iban));
            }
        }

        result.sort_by_key(|(code, _, _, _)| *code);
        result
    }
}
