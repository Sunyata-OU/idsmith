pub mod checksum;
pub mod date;
pub mod generic;

// Europe (existing)
pub mod at;
pub mod be;
pub mod bg;
pub mod ch;
pub mod cz;
pub mod de;
pub mod dk;
pub mod ee;
pub mod es;
pub mod fi;
pub mod fr;
pub mod gb;
pub mod gr;
pub mod hr;
pub mod ie;
pub mod is_;
pub mod it;
pub mod jmbg;
pub mod lv;
pub mod nl;
pub mod no;
pub mod pl;
pub mod pt;
pub mod ro;
pub mod se;
pub mod tr;

// Americas
pub mod ar;
pub mod br;
pub mod ca;
pub mod cl;
pub mod co;
pub mod ec;
pub mod mx;
pub mod pe;
pub mod us;
pub mod uy;

// Asia-Pacific
pub mod au;
pub mod cn;
pub mod hk;
pub mod id_;
pub mod in_;
pub mod jp;
pub mod kr;
pub mod my;
pub mod nz;
pub mod sg;
pub mod th;
pub mod tw;

// Africa/Middle East
pub mod eg;
pub mod il;
pub mod za;

use date::Gender;

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub gender: Option<Gender>,
    pub year: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct IdResult {
    pub code: String,
    pub gender: Option<String>,
    pub dob: Option<String>,
    pub valid: bool,
}

type GenerateFn = fn(&GenOptions, &mut rand::rngs::ThreadRng) -> String;
type ValidateFn = fn(&str) -> bool;
type ParseFn = fn(&str) -> IdResult;

struct RegistryEntry {
    code: &'static str,
    country_name: &'static str,
    name: &'static str,
    generate: GenerateFn,
    validate: ValidateFn,
    parse: ParseFn,
}

struct TerritoryAlias {
    code: &'static str,
    country_name: &'static str,
    parent_code: &'static str,
}

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
    TerritoryAlias { code: "BL", country_name: "Saint Barthelemy", parent_code: "FR" },
    // UK territories
    TerritoryAlias { code: "GS", country_name: "South Georgia and the South Sandwich Islands", parent_code: "GB" },
    TerritoryAlias { code: "IO", country_name: "British Indian Ocean Territory", parent_code: "GB" },
    TerritoryAlias { code: "SH", country_name: "Saint Helena, Ascension and Tristan da Cunha", parent_code: "GB" },
    // Norwegian territories
    TerritoryAlias { code: "BV", country_name: "Bouvet Island", parent_code: "NO" },
    TerritoryAlias { code: "SJ", country_name: "Svalbard and Jan Mayen", parent_code: "NO" },
    // Danish territories
    TerritoryAlias { code: "FO", country_name: "Faroe Islands", parent_code: "DK" },
    TerritoryAlias { code: "GL", country_name: "Greenland", parent_code: "DK" },
    // French territories
    TerritoryAlias { code: "GF", country_name: "French Guiana", parent_code: "FR" },
    TerritoryAlias { code: "GP", country_name: "Guadeloupe", parent_code: "FR" },
    TerritoryAlias { code: "MF", country_name: "Saint Martin", parent_code: "FR" },
    TerritoryAlias { code: "MQ", country_name: "Martinique", parent_code: "FR" },
    TerritoryAlias { code: "NC", country_name: "New Caledonia", parent_code: "FR" },
    TerritoryAlias { code: "PF", country_name: "French Polynesia", parent_code: "FR" },
    TerritoryAlias { code: "PM", country_name: "Saint Pierre and Miquelon", parent_code: "FR" },
    TerritoryAlias { code: "RE", country_name: "Reunion", parent_code: "FR" },
    TerritoryAlias { code: "TF", country_name: "French Southern Territories", parent_code: "FR" },
    TerritoryAlias { code: "WF", country_name: "Wallis and Futuna", parent_code: "FR" },
    TerritoryAlias { code: "YT", country_name: "Mayotte", parent_code: "FR" },
    // UK territory
    TerritoryAlias { code: "VG", country_name: "British Virgin Islands", parent_code: "GB" },
];

fn resolve_alias(code: &str) -> Option<&'static TerritoryAlias> {
    TERRITORY_ALIASES.iter().find(|a| a.code == code)
}

pub struct Registry {
    entries: Vec<RegistryEntry>,
}

// Wrapper functions for countries that share implementations but need
// concrete fn pointers (can't use closures as fn pointers with captures)
fn gen_si(opts: &GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    jmbg::generate_si(opts, rng)
}
fn gen_rs(opts: &GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    jmbg::generate_rs(opts, rng)
}
fn gen_ba(opts: &GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    jmbg::generate_ba(opts, rng)
}
fn gen_me(opts: &GenOptions, rng: &mut rand::rngs::ThreadRng) -> String {
    jmbg::generate_me(opts, rng)
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let entries = vec![
            // ── Europe (existing 31) ──
            RegistryEntry {
                code: "EE",
                country_name: "Estonia",
                name: "Isikukood",
                generate: ee::generate,
                validate: ee::validate,
                parse: ee::parse,
            },
            RegistryEntry {
                code: "LT",
                country_name: "Lithuania",
                name: "Asmens kodas",
                generate: ee::generate,
                validate: ee::validate,
                parse: ee::parse,
            },
            RegistryEntry {
                code: "LV",
                country_name: "Latvia",
                name: "Personas kods",
                generate: lv::generate,
                validate: lv::validate,
                parse: lv::parse,
            },
            RegistryEntry {
                code: "FI",
                country_name: "Finland",
                name: "Henkilotunnus",
                generate: fi::generate,
                validate: fi::validate,
                parse: fi::parse,
            },
            RegistryEntry {
                code: "SE",
                country_name: "Sweden",
                name: "Personnummer",
                generate: se::generate,
                validate: se::validate,
                parse: se::parse,
            },
            RegistryEntry {
                code: "NO",
                country_name: "Norway",
                name: "Fodselsnummer",
                generate: no::generate,
                validate: no::validate,
                parse: no::parse,
            },
            RegistryEntry {
                code: "DK",
                country_name: "Denmark",
                name: "CPR-nummer",
                generate: dk::generate,
                validate: dk::validate,
                parse: dk::parse,
            },
            RegistryEntry {
                code: "IS",
                country_name: "Iceland",
                name: "Kennitala",
                generate: is_::generate,
                validate: is_::validate,
                parse: is_::parse,
            },
            RegistryEntry {
                code: "DE",
                country_name: "Germany",
                name: "Steuerliche IdNr",
                generate: de::generate,
                validate: de::validate,
                parse: de::parse,
            },
            RegistryEntry {
                code: "AT",
                country_name: "Austria",
                name: "Sozialversicherungsnr",
                generate: at::generate,
                validate: at::validate,
                parse: at::parse,
            },
            RegistryEntry {
                code: "CH",
                country_name: "Switzerland",
                name: "AHV-Nummer",
                generate: ch::generate,
                validate: ch::validate,
                parse: ch::parse,
            },
            RegistryEntry {
                code: "NL",
                country_name: "Netherlands",
                name: "BSN",
                generate: nl::generate,
                validate: nl::validate,
                parse: nl::parse,
            },
            RegistryEntry {
                code: "BE",
                country_name: "Belgium",
                name: "Rijksregisternr",
                generate: be::generate,
                validate: be::validate,
                parse: be::parse,
            },
            RegistryEntry {
                code: "FR",
                country_name: "France",
                name: "NIR",
                generate: fr::generate,
                validate: fr::validate,
                parse: fr::parse,
            },
            RegistryEntry {
                code: "ES",
                country_name: "Spain",
                name: "DNI",
                generate: es::generate,
                validate: es::validate,
                parse: es::parse,
            },
            RegistryEntry {
                code: "PT",
                country_name: "Portugal",
                name: "NIF",
                generate: pt::generate,
                validate: pt::validate,
                parse: pt::parse,
            },
            RegistryEntry {
                code: "IT",
                country_name: "Italy",
                name: "Codice Fiscale",
                generate: it::generate,
                validate: it::validate,
                parse: it::parse,
            },
            RegistryEntry {
                code: "GB",
                country_name: "United Kingdom",
                name: "NINO",
                generate: gb::generate,
                validate: gb::validate,
                parse: gb::parse,
            },
            RegistryEntry {
                code: "IE",
                country_name: "Ireland",
                name: "PPS Number",
                generate: ie::generate,
                validate: ie::validate,
                parse: ie::parse,
            },
            RegistryEntry {
                code: "PL",
                country_name: "Poland",
                name: "PESEL",
                generate: pl::generate,
                validate: pl::validate,
                parse: pl::parse,
            },
            RegistryEntry {
                code: "CZ",
                country_name: "Czech Republic",
                name: "Rodne cislo",
                generate: cz::generate,
                validate: cz::validate,
                parse: cz::parse,
            },
            RegistryEntry {
                code: "SK",
                country_name: "Slovakia",
                name: "Rodne cislo",
                generate: cz::generate,
                validate: cz::validate,
                parse: cz::parse,
            },
            RegistryEntry {
                code: "RO",
                country_name: "Romania",
                name: "CNP",
                generate: ro::generate,
                validate: ro::validate,
                parse: ro::parse,
            },
            RegistryEntry {
                code: "BG",
                country_name: "Bulgaria",
                name: "EGN",
                generate: bg::generate,
                validate: bg::validate,
                parse: bg::parse,
            },
            RegistryEntry {
                code: "HR",
                country_name: "Croatia",
                name: "OIB",
                generate: hr::generate,
                validate: hr::validate,
                parse: hr::parse,
            },
            RegistryEntry {
                code: "SI",
                country_name: "Slovenia",
                name: "EMSO",
                generate: gen_si,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "RS",
                country_name: "Serbia",
                name: "JMBG",
                generate: gen_rs,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "BA",
                country_name: "Bosnia and Herzegovina",
                name: "JMBG",
                generate: gen_ba,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "ME",
                country_name: "Montenegro",
                name: "JMBG",
                generate: gen_me,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "TR",
                country_name: "Turkey",
                name: "TC Kimlik No",
                generate: tr::generate,
                validate: tr::validate,
                parse: tr::parse,
            },
            RegistryEntry {
                code: "GR",
                country_name: "Greece",
                name: "AMKA",
                generate: gr::generate,
                validate: gr::validate,
                parse: gr::parse,
            },
            // ── Americas (10 new) ──
            RegistryEntry {
                code: "US",
                country_name: "United States",
                name: "SSN",
                generate: us::generate,
                validate: us::validate,
                parse: us::parse,
            },
            RegistryEntry {
                code: "CA",
                country_name: "Canada",
                name: "SIN",
                generate: ca::generate,
                validate: ca::validate,
                parse: ca::parse,
            },
            RegistryEntry {
                code: "BR",
                country_name: "Brazil",
                name: "CPF",
                generate: br::generate,
                validate: br::validate,
                parse: br::parse,
            },
            RegistryEntry {
                code: "AR",
                country_name: "Argentina",
                name: "CUIL",
                generate: ar::generate,
                validate: ar::validate,
                parse: ar::parse,
            },
            RegistryEntry {
                code: "CL",
                country_name: "Chile",
                name: "RUT",
                generate: cl::generate,
                validate: cl::validate,
                parse: cl::parse,
            },
            RegistryEntry {
                code: "CO",
                country_name: "Colombia",
                name: "NIT",
                generate: co::generate,
                validate: co::validate,
                parse: co::parse,
            },
            RegistryEntry {
                code: "UY",
                country_name: "Uruguay",
                name: "CI",
                generate: uy::generate,
                validate: uy::validate,
                parse: uy::parse,
            },
            RegistryEntry {
                code: "EC",
                country_name: "Ecuador",
                name: "Cedula",
                generate: ec::generate,
                validate: ec::validate,
                parse: ec::parse,
            },
            RegistryEntry {
                code: "PE",
                country_name: "Peru",
                name: "DNI",
                generate: pe::generate,
                validate: pe::validate,
                parse: pe::parse,
            },
            RegistryEntry {
                code: "MX",
                country_name: "Mexico",
                name: "CURP",
                generate: mx::generate,
                validate: mx::validate,
                parse: mx::parse,
            },
            // ── Asia-Pacific (12 new) ──
            RegistryEntry {
                code: "CN",
                country_name: "China",
                name: "Resident ID",
                generate: cn::generate,
                validate: cn::validate,
                parse: cn::parse,
            },
            RegistryEntry {
                code: "IN",
                country_name: "India",
                name: "Aadhaar",
                generate: in_::generate,
                validate: in_::validate,
                parse: in_::parse,
            },
            RegistryEntry {
                code: "JP",
                country_name: "Japan",
                name: "My Number",
                generate: jp::generate,
                validate: jp::validate,
                parse: jp::parse,
            },
            RegistryEntry {
                code: "KR",
                country_name: "South Korea",
                name: "RRN",
                generate: kr::generate,
                validate: kr::validate,
                parse: kr::parse,
            },
            RegistryEntry {
                code: "TW",
                country_name: "Taiwan",
                name: "National ID",
                generate: tw::generate,
                validate: tw::validate,
                parse: tw::parse,
            },
            RegistryEntry {
                code: "TH",
                country_name: "Thailand",
                name: "Citizen ID",
                generate: th::generate,
                validate: th::validate,
                parse: th::parse,
            },
            RegistryEntry {
                code: "SG",
                country_name: "Singapore",
                name: "NRIC",
                generate: sg::generate,
                validate: sg::validate,
                parse: sg::parse,
            },
            RegistryEntry {
                code: "MY",
                country_name: "Malaysia",
                name: "MyKad",
                generate: my::generate,
                validate: my::validate,
                parse: my::parse,
            },
            RegistryEntry {
                code: "ID",
                country_name: "Indonesia",
                name: "NIK",
                generate: id_::generate,
                validate: id_::validate,
                parse: id_::parse,
            },
            RegistryEntry {
                code: "HK",
                country_name: "Hong Kong",
                name: "HKID",
                generate: hk::generate,
                validate: hk::validate,
                parse: hk::parse,
            },
            RegistryEntry {
                code: "AU",
                country_name: "Australia",
                name: "TFN",
                generate: au::generate,
                validate: au::validate,
                parse: au::parse,
            },
            RegistryEntry {
                code: "NZ",
                country_name: "New Zealand",
                name: "IRD",
                generate: nz::generate,
                validate: nz::validate,
                parse: nz::parse,
            },
            // ── Africa/Middle East (3 new) ──
            RegistryEntry {
                code: "ZA",
                country_name: "South Africa",
                name: "SA ID",
                generate: za::generate,
                validate: za::validate,
                parse: za::parse,
            },
            RegistryEntry {
                code: "IL",
                country_name: "Israel",
                name: "Teudat Zehut",
                generate: il::generate,
                validate: il::validate,
                parse: il::parse,
            },
            RegistryEntry {
                code: "EG",
                country_name: "Egypt",
                name: "National ID",
                generate: eg::generate,
                validate: eg::validate,
                parse: eg::parse,
            },
        ];
        Registry { entries }
    }

    fn find(&self, country: &str) -> Option<&RegistryEntry> {
        self.entries.iter().find(|e| e.code == country)
    }

    pub fn generate(
        &self,
        country: &str,
        opts: &GenOptions,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Option<String> {
        // Tier 1: specific implementation
        if let Some(entry) = self.find(country) {
            return Some((entry.generate)(opts, rng));
        }
        // Tier 2: generic fallback
        if let Some(id) = generic::generate(country, opts, rng) {
            return Some(id);
        }
        // Tier 3: territory alias
        if let Some(alias) = resolve_alias(country) {
            if let Some(entry) = self.find(alias.parent_code) {
                return Some((entry.generate)(opts, rng));
            }
            return generic::generate(alias.parent_code, opts, rng);
        }
        None
    }

    #[allow(dead_code)]
    pub fn validate(&self, country: &str, code: &str) -> Option<bool> {
        if let Some(entry) = self.find(country) {
            return Some((entry.validate)(code));
        }
        if let Some(valid) = generic::validate(country, code) {
            return Some(valid);
        }
        if let Some(alias) = resolve_alias(country) {
            if let Some(entry) = self.find(alias.parent_code) {
                return Some((entry.validate)(code));
            }
            return generic::validate(alias.parent_code, code);
        }
        None
    }

    pub fn parse(&self, country: &str, code: &str) -> Option<IdResult> {
        if let Some(entry) = self.find(country) {
            return Some((entry.parse)(code));
        }
        if let Some(result) = generic::parse(country, code) {
            return Some(result);
        }
        if let Some(alias) = resolve_alias(country) {
            if let Some(entry) = self.find(alias.parent_code) {
                return Some((entry.parse)(code));
            }
            return generic::parse(alias.parent_code, code);
        }
        None
    }

    pub fn name(&self, country: &str) -> Option<&str> {
        if let Some(e) = self.find(country) {
            return Some(e.name);
        }
        if let Some(name) = generic::id_name(country) {
            return Some(name);
        }
        if let Some(alias) = resolve_alias(country) {
            if let Some(e) = self.find(alias.parent_code) {
                return Some(e.name);
            }
            return generic::id_name(alias.parent_code);
        }
        None
    }

    pub fn country_name(&self, country: &str) -> Option<&str> {
        if let Some(e) = self.find(country) {
            return Some(e.country_name);
        }
        if let Some(name) = generic::country_name(country) {
            return Some(name);
        }
        if let Some(alias) = resolve_alias(country) {
            return Some(alias.country_name);
        }
        None
    }

    pub fn is_supported(&self, country: &str) -> bool {
        self.find(country).is_some()
            || generic::is_supported(country)
            || resolve_alias(country).is_some()
    }

    pub fn list_countries(&self) -> Vec<(&str, &str, &str)> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for e in &self.entries {
            if seen.insert(e.code) {
                result.push((e.code, e.country_name, e.name));
            }
        }
        for item in generic::list_countries() {
            if seen.insert(item.0) {
                result.push(item);
            }
        }
        for alias in TERRITORY_ALIASES {
            if seen.insert(alias.code) {
                let name = self
                    .find(alias.parent_code)
                    .map(|e| e.name)
                    .unwrap_or("National ID");
                result.push((alias.code, alias.country_name, name));
            }
        }
        result.sort_by_key(|(code, _, _)| *code);
        result
    }
}
