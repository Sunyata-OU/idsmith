pub mod checksum;
pub mod date;

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
    name: &'static str,
    generate: GenerateFn,
    validate: ValidateFn,
    parse: ParseFn,
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
        self.find(country).map(|e| (e.generate)(opts, rng))
    }

    #[allow(dead_code)]
    pub fn validate(&self, country: &str, code: &str) -> Option<bool> {
        self.find(country).map(|e| (e.validate)(code))
    }

    pub fn parse(&self, country: &str, code: &str) -> Option<IdResult> {
        self.find(country).map(|e| (e.parse)(code))
    }

    pub fn name(&self, country: &str) -> Option<&str> {
        self.find(country).map(|e| e.name)
    }

    pub fn list_countries(&self) -> Vec<(&str, &str)> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for e in &self.entries {
            if seen.insert(e.code) {
                result.push((e.code, e.name));
            }
        }
        result.sort_by_key(|(code, _)| *code);
        result
    }
}
