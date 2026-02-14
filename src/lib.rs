//! # idsmith
//!
//! Validate and generate checksum-correct IBANs, personal IDs, bank accounts,
//! credit cards, SWIFT/BIC, company IDs, driver's licenses, tax IDs, passports,
//! LEI codes, and EU VAT numbers.
//! Every identifier passes mod-97 (IBAN) and national checksum validation.
//!
//! ## Quick Start
//!
//! ```rust
//! use rand::thread_rng;
//! use idsmith::{iban, personal_id};
//!
//! let mut rng = thread_rng();
//!
//! // Generate a German IBAN
//! let code = iban::generate_iban(Some("DE"), &mut rng).unwrap();
//! println!("{}", iban::format_iban(&code));
//!
//! // Generate an Estonian personal ID
//! let registry = personal_id::Registry::new();
//! let opts = personal_id::GenOptions::default();
//! let id = registry.generate("EE", &opts, &mut rng).unwrap();
//! println!("{}", id);
//! ```

//! # idsmith
//!
//! A comprehensive **Generator** and **Validator** for valid, checksum-correct
//! identifiers. Supports IBANs, Personal IDs, Bank Accounts,
//! Credit Cards, SWIFT/BIC, Company IDs, Driver's Licenses, Tax IDs, Passports,
//! LEI codes, and EU VAT numbers.
//!
//! ## Quick Start (Generation)
//!
//! ```rust
//! use rand::thread_rng;
//! use idsmith::{bank_accounts, personal_ids};
//!
//! let mut rng = thread_rng();
//!
//! // Generate a German IBAN
//! let code = idsmith::iban::generate_iban(Some("DE"), &mut rng).unwrap();
//!
//! // Generate an Estonian personal ID
//! let id = personal_ids().generate("EE", &Default::default(), &mut rng).unwrap();
//! ```
//!
//! ## Quick Start (Validation)
//!
//! ```rust
//! use idsmith::{credit_cards, personal_ids};
//!
//! // Validate a credit card number
//! let is_valid = credit_cards().validate("4152839405126374");
//!
//! // Validate a US Social Security Number
//! let is_ssn_valid = personal_ids().validate("US", "446-72-2445").unwrap_or(false);
//! ```

pub mod bank_account;
pub mod company_id;
pub mod countries;
pub mod credit_card;
pub mod driver_license;
pub mod iban;
pub mod lei;
pub mod passport;
pub mod personal_id;
pub mod swift;
pub mod tax_id;
pub mod vat;

#[cfg(feature = "csv")]
pub mod csv;

use std::sync::OnceLock;

/// Global registry for bank accounts.
/// Provides methods to generate, validate, and format bank account numbers.
pub fn bank_accounts() -> &'static bank_account::Registry {
    static REGISTRY: OnceLock<bank_account::Registry> = OnceLock::new();
    REGISTRY.get_or_init(bank_account::Registry::new)
}

/// Global registry for personal IDs.
/// Provides methods to generate, validate, and parse national ID numbers (SSN, CPF, Aadhaar, etc.).
pub fn personal_ids() -> &'static personal_id::Registry {
    static REGISTRY: OnceLock<personal_id::Registry> = OnceLock::new();
    REGISTRY.get_or_init(personal_id::Registry::new)
}

/// Global registry for credit cards.
/// Provides methods to generate and validate credit card numbers for major brands (Visa, Mastercard, Amex, etc.).
pub fn credit_cards() -> &'static credit_card::Registry {
    static REGISTRY: OnceLock<credit_card::Registry> = OnceLock::new();
    REGISTRY.get_or_init(credit_card::Registry::new)
}

/// Global registry for company IDs.
/// Provides methods to generate and validate business identifiers (VAT, EIN, CIF) for all countries.
pub fn company_ids() -> &'static company_id::Registry {
    static REGISTRY: OnceLock<company_id::Registry> = OnceLock::new();
    REGISTRY.get_or_init(company_id::Registry::new)
}

/// Global registry for SWIFT codes.
/// Provides methods to generate and validate 8 and 11 character SWIFT/BIC codes.
pub fn swift_codes() -> &'static swift::Registry {
    static REGISTRY: OnceLock<swift::Registry> = OnceLock::new();
    REGISTRY.get_or_init(swift::Registry::new)
}

/// Global registry for driver's licenses.
/// Provides methods to generate and validate driver's license numbers for all countries.
pub fn driver_licenses() -> &'static driver_license::Registry {
    static REGISTRY: OnceLock<driver_license::Registry> = OnceLock::new();
    REGISTRY.get_or_init(driver_license::Registry::new)
}

/// Global registry for tax IDs.
/// Provides methods to generate and validate tax identification numbers (PAN, TIN, etc.) for all countries.
pub fn tax_ids() -> &'static tax_id::Registry {
    static REGISTRY: OnceLock<tax_id::Registry> = OnceLock::new();
    REGISTRY.get_or_init(tax_id::Registry::new)
}

/// Global registry for LEI (Legal Entity Identifier) codes.
/// Provides methods to generate and validate ISO 17442 LEI codes.
pub fn lei_codes() -> &'static lei::Registry {
    static REGISTRY: OnceLock<lei::Registry> = OnceLock::new();
    REGISTRY.get_or_init(lei::Registry::new)
}

/// Global registry for passport numbers.
/// Provides methods to generate and validate passport numbers for all countries.
pub fn passports() -> &'static passport::Registry {
    static REGISTRY: OnceLock<passport::Registry> = OnceLock::new();
    REGISTRY.get_or_init(passport::Registry::new)
}

/// Global registry for VAT numbers.
/// Provides methods to generate and validate EU VAT identification numbers.
pub fn vat_ids() -> &'static vat::Registry {
    static REGISTRY: OnceLock<vat::Registry> = OnceLock::new();
    REGISTRY.get_or_init(vat::Registry::new)
}
