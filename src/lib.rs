//! # EU Test Data Generator
//!
//! Generate valid, checksum-correct IBANs and personal ID codes for European
//! and worldwide countries. Every generated code passes mod-97 (IBAN) and
//! national checksum validation.
//!
//! ## Quick Start
//!
//! ```rust
//! use rand::thread_rng;
//! use eu_test_data_generator::{iban, personal_id};
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

pub mod iban;
pub mod personal_id;
