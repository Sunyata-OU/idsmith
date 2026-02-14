use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::thread_rng;

// ── Result types ──

#[napi(object)]
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

impl From<idsmith::bank_account::AccountResult> for AccountResult {
    fn from(r: idsmith::bank_account::AccountResult) -> Self {
        Self {
            country_code: r.country_code,
            country_name: r.country_name,
            format_name: r.format_name,
            bank_code: r.bank_code,
            branch_code: r.branch_code,
            account_number: r.account_number,
            check_digits: r.check_digits,
            formatted: r.formatted,
            raw: r.raw,
            iban: r.iban,
            valid: r.valid,
        }
    }
}

#[napi(object)]
pub struct IdResult {
    pub country_code: String,
    pub code: String,
    pub gender: Option<String>,
    pub dob: Option<String>,
    pub valid: bool,
}

impl From<idsmith::personal_id::IdResult> for IdResult {
    fn from(r: idsmith::personal_id::IdResult) -> Self {
        Self {
            country_code: r.country_code,
            code: r.code,
            gender: r.gender,
            dob: r.dob,
            valid: r.valid,
        }
    }
}

#[napi(object)]
pub struct CardResult {
    pub brand: String,
    pub number: String,
    pub formatted: String,
    pub cvv: String,
    pub expiry: String,
    pub valid: bool,
}

impl From<idsmith::credit_card::CardResult> for CardResult {
    fn from(r: idsmith::credit_card::CardResult) -> Self {
        Self {
            brand: r.brand,
            number: r.number,
            formatted: r.formatted,
            cvv: r.cvv,
            expiry: r.expiry,
            valid: r.valid,
        }
    }
}

#[napi(object)]
pub struct CompanyResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub valid: bool,
}

impl From<idsmith::company_id::CompanyResult> for CompanyResult {
    fn from(r: idsmith::company_id::CompanyResult) -> Self {
        Self {
            country_code: r.country_code,
            country_name: r.country_name,
            name: r.name,
            code: r.code,
            valid: r.valid,
        }
    }
}

#[napi(object)]
pub struct SwiftResult {
    pub code: String,
    pub bank: String,
    pub country: String,
    pub location: String,
    pub branch: Option<String>,
    pub valid: bool,
}

impl From<idsmith::swift::SwiftResult> for SwiftResult {
    fn from(r: idsmith::swift::SwiftResult) -> Self {
        Self {
            code: r.code,
            bank: r.bank,
            country: r.country,
            location: r.location,
            branch: r.branch,
            valid: r.valid,
        }
    }
}

#[napi(object)]
pub struct BankCountryInfo {
    pub code: String,
    pub name: String,
    pub format: String,
    pub has_iban: bool,
}

#[napi(object)]
pub struct CountryInfo {
    pub code: String,
    pub country_name: String,
    pub id_name: String,
}

// ── BankAccount ──

#[napi]
pub struct BankAccount;

#[napi]
impl BankAccount {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: String, bank_code: Option<String>) -> Result<AccountResult> {
        let mut rng = thread_rng();
        let opts = idsmith::bank_account::GenOptions { bank_code };
        idsmith::bank_accounts()
            .generate(&country, &opts, &mut rng)
            .map(AccountResult::from)
            .ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    format!("Unsupported country: {}", country),
                )
            })
    }

    #[napi]
    pub fn validate(country: String, raw: String) -> Result<bool> {
        idsmith::bank_accounts()
            .validate(&country, &raw)
            .ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    format!("Unsupported country: {}", country),
                )
            })
    }

    #[napi]
    pub fn format(country: String, raw: String) -> Result<String> {
        idsmith::bank_accounts()
            .format(&country, &raw)
            .ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    format!("Unsupported country: {}", country),
                )
            })
    }

    #[napi]
    pub fn list_countries() -> Vec<BankCountryInfo> {
        idsmith::bank_accounts()
            .list_countries()
            .iter()
            .map(|(code, name, format, has_iban)| BankCountryInfo {
                code: code.to_string(),
                name: name.to_string(),
                format: format.to_string(),
                has_iban: *has_iban,
            })
            .collect()
    }

    #[napi]
    pub fn is_supported(country: String) -> bool {
        idsmith::bank_accounts().is_supported(&country)
    }
}

// ── PersonalId ──

#[napi]
pub struct PersonalId;

#[napi]
impl PersonalId {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: String, gender: Option<String>, year: Option<u32>) -> Result<String> {
        let mut rng = thread_rng();
        let g = idsmith::personal_id::date::Gender::from_str_opt(gender.as_deref());
        let opts = idsmith::personal_id::GenOptions {
            gender: g,
            year: year.map(|y| y as u16),
        };
        idsmith::personal_ids()
            .generate(&country, &opts, &mut rng)
            .ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    format!("Unsupported country: {}", country),
                )
            })
    }

    #[napi]
    pub fn validate(country: String, code: String) -> Result<bool> {
        idsmith::personal_ids()
            .validate(&country, &code)
            .ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    format!("Unsupported country: {}", country),
                )
            })
    }

    #[napi]
    pub fn parse(country: String, code: String) -> Result<IdResult> {
        idsmith::personal_ids()
            .parse(&country, &code)
            .map(IdResult::from)
            .ok_or_else(|| {
                Error::new(
                    Status::InvalidArg,
                    format!("Unsupported country: {}", country),
                )
            })
    }

    #[napi]
    pub fn list_countries() -> Vec<CountryInfo> {
        idsmith::personal_ids()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| CountryInfo {
                code: code.to_string(),
                country_name: country_name.to_string(),
                id_name: id_name.to_string(),
            })
            .collect()
    }

    #[napi]
    pub fn is_supported(country: String) -> bool {
        idsmith::personal_ids().is_supported(&country)
    }
}

// ── CreditCard ──

#[napi]
pub struct CreditCard;

#[napi]
impl CreditCard {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(brand: Option<String>) -> Result<CardResult> {
        let mut rng = thread_rng();
        let opts = idsmith::credit_card::GenOptions { brand };
        idsmith::credit_cards()
            .generate(&opts, &mut rng)
            .map(CardResult::from)
            .ok_or_else(|| Error::new(Status::GenericFailure, "Failed to generate credit card"))
    }

    #[napi]
    pub fn validate(number: String) -> bool {
        idsmith::credit_cards().validate(&number)
    }

    #[napi]
    pub fn format(brand: String, number: String) -> String {
        idsmith::credit_cards().format(&brand, &number)
    }

    #[napi]
    pub fn list_brands() -> Vec<String> {
        idsmith::credit_cards()
            .list_brands()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
}

// ── CompanyId ──

#[napi]
pub struct CompanyId;

#[napi]
impl CompanyId {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: Option<String>) -> Result<CompanyResult> {
        let mut rng = thread_rng();
        let opts = idsmith::company_id::GenOptions { country };
        idsmith::company_ids()
            .generate(&opts, &mut rng)
            .map(CompanyResult::from)
            .ok_or_else(|| Error::new(Status::GenericFailure, "Failed to generate company ID"))
    }

    #[napi]
    pub fn validate(country: String, code: String) -> bool {
        idsmith::company_ids().validate(&country, &code)
    }

    #[napi]
    pub fn list_countries() -> Vec<CountryInfo> {
        idsmith::company_ids()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| CountryInfo {
                code: code.to_string(),
                country_name: country_name.to_string(),
                id_name: id_name.to_string(),
            })
            .collect()
    }
}

// ── Swift ──

#[napi]
pub struct Swift;

#[napi]
impl Swift {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: Option<String>) -> SwiftResult {
        let mut rng = thread_rng();
        let opts = idsmith::swift::GenOptions { country };
        idsmith::swift_codes().generate(&opts, &mut rng).into()
    }

    #[napi]
    pub fn validate(code: String) -> bool {
        idsmith::swift_codes().validate(&code)
    }
}

// ── DriverLicense ──

#[napi(object)]
pub struct DriverLicenseResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub state: Option<String>,
    pub valid: bool,
}

impl From<idsmith::driver_license::DriverLicenseResult> for DriverLicenseResult {
    fn from(r: idsmith::driver_license::DriverLicenseResult) -> Self {
        Self {
            country_code: r.country_code,
            country_name: r.country_name,
            name: r.name,
            code: r.code,
            state: r.state,
            valid: r.valid,
        }
    }
}

#[napi]
pub struct DriverLicense;

#[napi]
impl DriverLicense {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: Option<String>, state: Option<String>) -> Result<DriverLicenseResult> {
        let mut rng = thread_rng();
        let opts = idsmith::driver_license::GenOptions { country, state };
        idsmith::driver_licenses()
            .generate(&opts, &mut rng)
            .map(DriverLicenseResult::from)
            .ok_or_else(|| {
                Error::new(
                    Status::GenericFailure,
                    "Failed to generate driver's license",
                )
            })
    }

    #[napi]
    pub fn validate(country: String, code: String) -> bool {
        idsmith::driver_licenses().validate(&country, &code)
    }

    #[napi]
    pub fn list_countries() -> Vec<CountryInfo> {
        idsmith::driver_licenses()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| CountryInfo {
                code: code.to_string(),
                country_name: country_name.to_string(),
                id_name: id_name.to_string(),
            })
            .collect()
    }
}

// ── TaxId ──

#[napi(object)]
pub struct TaxIdResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub holder_type: Option<String>,
    pub valid: bool,
}

impl From<idsmith::tax_id::TaxIdResult> for TaxIdResult {
    fn from(r: idsmith::tax_id::TaxIdResult) -> Self {
        Self {
            country_code: r.country_code,
            country_name: r.country_name,
            name: r.name,
            code: r.code,
            holder_type: r.holder_type,
            valid: r.valid,
        }
    }
}

#[napi]
pub struct TaxId;

#[napi]
impl TaxId {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: Option<String>, holder_type: Option<String>) -> Result<TaxIdResult> {
        let mut rng = thread_rng();
        let opts = idsmith::tax_id::GenOptions {
            country,
            holder_type,
        };
        idsmith::tax_ids()
            .generate(&opts, &mut rng)
            .map(TaxIdResult::from)
            .ok_or_else(|| Error::new(Status::GenericFailure, "Failed to generate tax ID"))
    }

    #[napi]
    pub fn validate(country: String, code: String) -> bool {
        idsmith::tax_ids().validate(&country, &code)
    }

    #[napi]
    pub fn list_countries() -> Vec<CountryInfo> {
        idsmith::tax_ids()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| CountryInfo {
                code: code.to_string(),
                country_name: country_name.to_string(),
                id_name: id_name.to_string(),
            })
            .collect()
    }
}

// ── Passport ──

#[napi(object)]
pub struct PassportResult {
    pub country_code: String,
    pub country_name: String,
    pub name: String,
    pub code: String,
    pub valid: bool,
}

impl From<idsmith::passport::PassportResult> for PassportResult {
    fn from(r: idsmith::passport::PassportResult) -> Self {
        Self {
            country_code: r.country_code,
            country_name: r.country_name,
            name: r.name,
            code: r.code,
            valid: r.valid,
        }
    }
}

#[napi]
pub struct Passport;

#[napi]
impl Passport {
    #[napi(factory)]
    pub fn create() -> Self {
        Self
    }

    #[napi]
    pub fn generate(country: Option<String>) -> Result<PassportResult> {
        let mut rng = thread_rng();
        let opts = idsmith::passport::GenOptions { country };
        idsmith::passports()
            .generate(&opts, &mut rng)
            .map(PassportResult::from)
            .ok_or_else(|| Error::new(Status::GenericFailure, "Failed to generate passport"))
    }

    #[napi]
    pub fn validate(country: String, code: String) -> bool {
        idsmith::passports().validate(&country, &code)
    }

    #[napi]
    pub fn list_countries() -> Vec<CountryInfo> {
        idsmith::passports()
            .list_countries()
            .iter()
            .map(|(code, country_name, id_name)| CountryInfo {
                code: code.to_string(),
                country_name: country_name.to_string(),
                id_name: id_name.to_string(),
            })
            .collect()
    }
}

// ── IBAN functions ──

#[napi]
pub fn generate_iban(country: Option<String>) -> Result<String> {
    let mut rng = thread_rng();
    idsmith::iban::generate_iban(country.as_deref(), &mut rng)
        .map_err(|e| Error::new(Status::InvalidArg, e))
}

#[napi]
pub fn validate_iban(iban: String) -> bool {
    idsmith::iban::validate_iban(&iban)
}

#[napi]
pub fn format_iban(iban: String) -> String {
    idsmith::iban::format_iban(&iban)
}

#[napi]
pub fn iban_countries() -> Vec<String> {
    idsmith::iban::supported_countries()
        .iter()
        .map(|s| s.to_string())
        .collect()
}
