pub mod checksum;
pub mod date;

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
pub mod cu;
pub mod do_;
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
pub mod dz;
pub mod eg;
pub mod il;
pub mod mu;
pub mod pk;
pub mod za;

use date::Gender;
#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct GenOptions {
    pub gender: Option<Gender>,
    pub year: Option<u16>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct IdResult {
    pub country_code: String,
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
    name: &'static str,
    generate: GenerateFn,
    validate: ValidateFn,
    parse: ParseFn,
}

struct TerritoryAlias {
    code: &'static str,
    parent_code: &'static str,
}

static TERRITORY_ALIASES: &[TerritoryAlias] = &[
    // US territories
    TerritoryAlias {
        code: "AS",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "GU",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "MP",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "PR",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "UM",
        parent_code: "US",
    },
    TerritoryAlias {
        code: "VI",
        parent_code: "US",
    },
    // Australian territories
    TerritoryAlias {
        code: "CC",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "CX",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "HM",
        parent_code: "AU",
    },
    TerritoryAlias {
        code: "NF",
        parent_code: "AU",
    },
    // NZ territories
    TerritoryAlias {
        code: "CK",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "NU",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "PN",
        parent_code: "NZ",
    },
    TerritoryAlias {
        code: "TK",
        parent_code: "NZ",
    },
    // French territory
    TerritoryAlias {
        code: "BL",
        parent_code: "FR",
    },
    // UK territories
    TerritoryAlias {
        code: "GS",
        parent_code: "GB",
    },
    TerritoryAlias {
        code: "IO",
        parent_code: "GB",
    },
    TerritoryAlias {
        code: "SH",
        parent_code: "GB",
    },
    // Norwegian territories
    TerritoryAlias {
        code: "BV",
        parent_code: "NO",
    },
    TerritoryAlias {
        code: "SJ",
        parent_code: "NO",
    },
    // Danish territories
    TerritoryAlias {
        code: "FO",
        parent_code: "DK",
    },
    TerritoryAlias {
        code: "GL",
        parent_code: "DK",
    },
    // French territories
    TerritoryAlias {
        code: "GF",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "GP",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "MF",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "MQ",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "NC",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "PF",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "PM",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "RE",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "TF",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "WF",
        parent_code: "FR",
    },
    TerritoryAlias {
        code: "YT",
        parent_code: "FR",
    },
    // UK territory
    TerritoryAlias {
        code: "VG",
        parent_code: "GB",
    },
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
                name: "Isikukood",
                generate: ee::generate,
                validate: ee::validate,
                parse: ee::parse,
            },
            RegistryEntry {
                code: "LT",
                name: "Asmens kodas",
                generate: ee::generate,
                validate: ee::validate,
                parse: ee::parse,
            },
            RegistryEntry {
                code: "LV",
                name: "Personas kods",
                generate: lv::generate,
                validate: lv::validate,
                parse: lv::parse,
            },
            RegistryEntry {
                code: "FI",
                name: "Henkilotunnus",
                generate: fi::generate,
                validate: fi::validate,
                parse: fi::parse,
            },
            RegistryEntry {
                code: "SE",
                name: "Personnummer",
                generate: se::generate,
                validate: se::validate,
                parse: se::parse,
            },
            RegistryEntry {
                code: "NO",
                name: "Fodselsnummer",
                generate: no::generate,
                validate: no::validate,
                parse: no::parse,
            },
            RegistryEntry {
                code: "DK",
                name: "CPR-nummer",
                generate: dk::generate,
                validate: dk::validate,
                parse: dk::parse,
            },
            RegistryEntry {
                code: "IS",
                name: "Kennitala",
                generate: is_::generate,
                validate: is_::validate,
                parse: is_::parse,
            },
            RegistryEntry {
                code: "DE",
                name: "Steuerliche IdNr",
                generate: de::generate,
                validate: de::validate,
                parse: de::parse,
            },
            RegistryEntry {
                code: "AT",
                name: "Sozialversicherungsnr",
                generate: at::generate,
                validate: at::validate,
                parse: at::parse,
            },
            RegistryEntry {
                code: "CH",
                name: "AHV-Nummer",
                generate: ch::generate,
                validate: ch::validate,
                parse: ch::parse,
            },
            RegistryEntry {
                code: "NL",
                name: "BSN",
                generate: nl::generate,
                validate: nl::validate,
                parse: nl::parse,
            },
            RegistryEntry {
                code: "BE",
                name: "Rijksregisternr",
                generate: be::generate,
                validate: be::validate,
                parse: be::parse,
            },
            RegistryEntry {
                code: "FR",
                name: "NIR",
                generate: fr::generate,
                validate: fr::validate,
                parse: fr::parse,
            },
            RegistryEntry {
                code: "ES",
                name: "DNI",
                generate: es::generate,
                validate: es::validate,
                parse: es::parse,
            },
            RegistryEntry {
                code: "PT",
                name: "NIF",
                generate: pt::generate,
                validate: pt::validate,
                parse: pt::parse,
            },
            RegistryEntry {
                code: "IT",
                name: "Codice Fiscale",
                generate: it::generate,
                validate: it::validate,
                parse: it::parse,
            },
            RegistryEntry {
                code: "GB",
                name: "NINO",
                generate: gb::generate,
                validate: gb::validate,
                parse: gb::parse,
            },
            RegistryEntry {
                code: "IE",
                name: "PPS Number",
                generate: ie::generate,
                validate: ie::validate,
                parse: ie::parse,
            },
            RegistryEntry {
                code: "PL",
                name: "PESEL",
                generate: pl::generate,
                validate: pl::validate,
                parse: pl::parse,
            },
            RegistryEntry {
                code: "CZ",
                name: "Rodne cislo",
                generate: cz::generate,
                validate: cz::validate,
                parse: cz::parse,
            },
            RegistryEntry {
                code: "SK",
                name: "Rodne cislo",
                generate: cz::generate,
                validate: cz::validate,
                parse: cz::parse,
            },
            RegistryEntry {
                code: "RO",
                name: "CNP",
                generate: ro::generate,
                validate: ro::validate,
                parse: ro::parse,
            },
            RegistryEntry {
                code: "BG",
                name: "EGN",
                generate: bg::generate,
                validate: bg::validate,
                parse: bg::parse,
            },
            RegistryEntry {
                code: "HR",
                name: "OIB",
                generate: hr::generate,
                validate: hr::validate,
                parse: hr::parse,
            },
            RegistryEntry {
                code: "SI",
                name: "EMSO",
                generate: gen_si,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "RS",
                name: "JMBG",
                generate: gen_rs,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "BA",
                name: "JMBG",
                generate: gen_ba,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "ME",
                name: "JMBG",
                generate: gen_me,
                validate: jmbg::validate,
                parse: jmbg::parse,
            },
            RegistryEntry {
                code: "TR",
                name: "TC Kimlik No",
                generate: tr::generate,
                validate: tr::validate,
                parse: tr::parse,
            },
            RegistryEntry {
                code: "GR",
                name: "AMKA",
                generate: gr::generate,
                validate: gr::validate,
                parse: gr::parse,
            },
            // ── Americas (10 new) ──
            RegistryEntry {
                code: "US",
                name: "SSN",
                generate: us::generate,
                validate: us::validate,
                parse: us::parse,
            },
            RegistryEntry {
                code: "CA",
                name: "SIN",
                generate: ca::generate,
                validate: ca::validate,
                parse: ca::parse,
            },
            RegistryEntry {
                code: "BR",
                name: "CPF",
                generate: br::generate,
                validate: br::validate,
                parse: br::parse,
            },
            RegistryEntry {
                code: "AR",
                name: "CUIL",
                generate: ar::generate,
                validate: ar::validate,
                parse: ar::parse,
            },
            RegistryEntry {
                code: "CL",
                name: "RUT",
                generate: cl::generate,
                validate: cl::validate,
                parse: cl::parse,
            },
            RegistryEntry {
                code: "CO",
                name: "NIT",
                generate: co::generate,
                validate: co::validate,
                parse: co::parse,
            },
            RegistryEntry {
                code: "UY",
                name: "CI",
                generate: uy::generate,
                validate: uy::validate,
                parse: uy::parse,
            },
            RegistryEntry {
                code: "CU",
                name: "NI",
                generate: cu::generate,
                validate: cu::validate,
                parse: cu::parse,
            },
            RegistryEntry {
                code: "DO",
                name: "Cedula",
                generate: do_::generate,
                validate: do_::validate,
                parse: do_::parse,
            },
            RegistryEntry {
                code: "EC",
                name: "Cedula",
                generate: ec::generate,
                validate: ec::validate,
                parse: ec::parse,
            },
            RegistryEntry {
                code: "PE",
                name: "DNI",
                generate: pe::generate,
                validate: pe::validate,
                parse: pe::parse,
            },
            RegistryEntry {
                code: "MX",
                name: "CURP",
                generate: mx::generate,
                validate: mx::validate,
                parse: mx::parse,
            },
            // ── Asia-Pacific (12 new) ──
            RegistryEntry {
                code: "CN",
                name: "Resident ID",
                generate: cn::generate,
                validate: cn::validate,
                parse: cn::parse,
            },
            RegistryEntry {
                code: "IN",
                name: "Aadhaar",
                generate: in_::generate,
                validate: in_::validate,
                parse: in_::parse,
            },
            RegistryEntry {
                code: "JP",
                name: "My Number",
                generate: jp::generate,
                validate: jp::validate,
                parse: jp::parse,
            },
            RegistryEntry {
                code: "KR",
                name: "RRN",
                generate: kr::generate,
                validate: kr::validate,
                parse: kr::parse,
            },
            RegistryEntry {
                code: "TW",
                name: "National ID",
                generate: tw::generate,
                validate: tw::validate,
                parse: tw::parse,
            },
            RegistryEntry {
                code: "TH",
                name: "Citizen ID",
                generate: th::generate,
                validate: th::validate,
                parse: th::parse,
            },
            RegistryEntry {
                code: "SG",
                name: "NRIC",
                generate: sg::generate,
                validate: sg::validate,
                parse: sg::parse,
            },
            RegistryEntry {
                code: "MY",
                name: "MyKad",
                generate: my::generate,
                validate: my::validate,
                parse: my::parse,
            },
            RegistryEntry {
                code: "ID",
                name: "NIK",
                generate: id_::generate,
                validate: id_::validate,
                parse: id_::parse,
            },
            RegistryEntry {
                code: "HK",
                name: "HKID",
                generate: hk::generate,
                validate: hk::validate,
                parse: hk::parse,
            },
            RegistryEntry {
                code: "AU",
                name: "TFN",
                generate: au::generate,
                validate: au::validate,
                parse: au::parse,
            },
            RegistryEntry {
                code: "NZ",
                name: "IRD",
                generate: nz::generate,
                validate: nz::validate,
                parse: nz::parse,
            },
            // ── Africa/Middle East (3 new) ──
            RegistryEntry {
                code: "ZA",
                name: "SA ID",
                generate: za::generate,
                validate: za::validate,
                parse: za::parse,
            },
            RegistryEntry {
                code: "IL",
                name: "Teudat Zehut",
                generate: il::generate,
                validate: il::validate,
                parse: il::parse,
            },
            RegistryEntry {
                code: "EG",
                name: "National ID",
                generate: eg::generate,
                validate: eg::validate,
                parse: eg::parse,
            },
            RegistryEntry {
                code: "DZ",
                name: "NIF",
                generate: dz::generate,
                validate: dz::validate,
                parse: dz::parse,
            },
            RegistryEntry {
                code: "MU",
                name: "NID",
                generate: mu::generate,
                validate: mu::validate,
                parse: mu::parse,
            },
            RegistryEntry {
                code: "PK",
                name: "CNIC",
                generate: pk::generate,
                validate: pk::validate,
                parse: pk::parse,
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
        if let Some(entry) = self.find(country) {
            return Some((entry.generate)(opts, rng));
        }
        if let Some(alias) = resolve_alias(country) {
            if let Some(entry) = self.find(alias.parent_code) {
                return Some((entry.generate)(opts, rng));
            }
        }
        None
    }

    pub fn validate(&self, country: &str, code: &str) -> Option<bool> {
        if let Some(entry) = self.find(country) {
            return Some((entry.validate)(code));
        }
        if let Some(alias) = resolve_alias(country) {
            if let Some(entry) = self.find(alias.parent_code) {
                return Some((entry.validate)(code));
            }
        }
        None
    }

    pub fn parse(&self, country: &str, code: &str) -> Option<IdResult> {
        let mut result = if let Some(entry) = self.find(country) {
            (entry.parse)(code)
        } else if let Some(alias) = resolve_alias(country) {
            if let Some(entry) = self.find(alias.parent_code) {
                (entry.parse)(code)
            } else {
                return None;
            }
        } else {
            return None;
        };
        result.country_code = country.to_string();
        Some(result)
    }

    pub fn name(&self, country: &str) -> Option<&str> {
        if let Some(e) = self.find(country) {
            return Some(e.name);
        }
        if let Some(alias) = resolve_alias(country) {
            if let Some(e) = self.find(alias.parent_code) {
                return Some(e.name);
            }
        }
        None
    }

    pub fn country_name(&self, country: &str) -> Option<&str> {
        crate::countries::get_country_name(country)
    }

    pub fn is_supported(&self, country: &str) -> bool {
        self.find(country).is_some() || resolve_alias(country).is_some()
    }

    pub fn list_countries(&self) -> Vec<(&str, &str, &str)> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for e in &self.entries {
            if seen.insert(e.code) {
                result.push((
                    e.code,
                    crate::countries::get_country_name(e.code).unwrap_or("Unknown"),
                    e.name,
                ));
            }
        }
        for alias in TERRITORY_ALIASES {
            if seen.insert(alias.code) {
                let name = self
                    .find(alias.parent_code)
                    .map(|e| e.name)
                    .unwrap_or("National ID");
                result.push((
                    alias.code,
                    crate::countries::get_country_name(alias.code).unwrap_or("Unknown"),
                    name,
                ));
            }
        }
        result.sort_by_key(|(code, _, _)| *code);
        result
    }
}
