//! Optional CSV formatting for IBAN and personal ID results.
//!
//! Enable with the `csv` feature flag:
//! ```toml
//! idsmith = { version = "...", features = ["csv"] }
//! ```

use std::io::Write;

use crate::bank_account::AccountResult;
use crate::company_id::CompanyResult;
use crate::credit_card::CardResult;
use crate::driver_license::DriverLicenseResult;
use crate::passport::PassportResult;
use crate::personal_id::IdResult;
use crate::swift::SwiftResult;
use crate::tax_id::TaxIdResult;

/// Wrap a CSV field in double quotes if it contains commas, double-quotes, or
/// newlines, per RFC 4180. Internal double-quotes are escaped by doubling.
fn csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// CSV header for IBAN rows.
pub const IBAN_HEADER: &str = "country,iban,iban_formatted,valid";

/// CSV header for personal ID rows.
pub const ID_HEADER: &str = "country,id_name,code,gender,dob,valid";

/// CSV header for credit card rows.
pub const CARD_HEADER: &str = "brand,number,formatted,cvv,expiry,valid";

/// CSV header for SWIFT rows.
pub const SWIFT_HEADER: &str = "country,bank,location,branch,code,valid";

/// CSV header for company ID rows.
pub const COMPANY_HEADER: &str = "country,id_name,code,valid";

/// Format a single IBAN as a CSV row.
pub fn iban_row(iban_code: &str, formatted: &str, valid: bool) -> String {
    format!(
        "{},{},{},{}",
        csv_field(&iban_code[..2]),
        csv_field(iban_code),
        csv_field(formatted),
        valid
    )
}

/// Format a single personal ID result as a CSV row.
pub fn id_row(country: &str, id_name: &str, result: &IdResult) -> String {
    format!(
        "{},{},{},{},{},{}",
        csv_field(country),
        csv_field(id_name),
        csv_field(&result.code),
        csv_field(result.gender.as_deref().unwrap_or("")),
        csv_field(result.dob.as_deref().unwrap_or("")),
        result.valid
    )
}

/// Format a single credit card result as a CSV row.
pub fn card_row(result: &CardResult) -> String {
    format!(
        "{},{},{},{},{},{}",
        csv_field(&result.brand),
        csv_field(&result.number),
        csv_field(&result.formatted),
        csv_field(&result.cvv),
        csv_field(&result.expiry),
        result.valid
    )
}

/// Format a single SWIFT result as a CSV row.
pub fn swift_row(result: &SwiftResult) -> String {
    format!(
        "{},{},{},{},{},{}",
        csv_field(&result.country),
        csv_field(&result.bank),
        csv_field(&result.location),
        csv_field(result.branch.as_deref().unwrap_or("")),
        csv_field(&result.code),
        result.valid
    )
}

/// Format a single company ID result as a CSV row.
pub fn company_row(result: &CompanyResult) -> String {
    format!(
        "{},{},{},{}",
        csv_field(&result.country_code),
        csv_field(&result.name),
        csv_field(&result.code),
        result.valid
    )
}

/// Write IBAN CSV header + rows to any [`Write`] destination.
pub fn write_iban_csv<W: Write>(
    writer: &mut W,
    rows: &[(String, String, bool)],
) -> std::io::Result<()> {
    writeln!(writer, "{}", IBAN_HEADER)?;
    for (iban_code, formatted, valid) in rows {
        writeln!(writer, "{}", iban_row(iban_code, formatted, *valid))?;
    }
    Ok(())
}

/// Write personal ID CSV header + rows to any [`Write`] destination.
pub fn write_id_csv<W: Write>(
    writer: &mut W,
    country: &str,
    id_name: &str,
    results: &[IdResult],
) -> std::io::Result<()> {
    writeln!(writer, "{}", ID_HEADER)?;
    for result in results {
        writeln!(writer, "{}", id_row(country, id_name, result))?;
    }
    Ok(())
}

/// CSV header for bank account rows.
pub const ACCOUNT_HEADER: &str =
    "country,country_name,format,bank_code,branch_code,account_number,check_digits,formatted,raw,iban,valid";

/// Format a single bank account result as a CSV row.
pub fn account_row(result: &AccountResult) -> String {
    format!(
        "{},{},{},{},{},{},{},{},{},{},{}",
        csv_field(&result.country_code),
        csv_field(&result.country_name),
        csv_field(&result.format_name),
        csv_field(result.bank_code.as_deref().unwrap_or("")),
        csv_field(result.branch_code.as_deref().unwrap_or("")),
        csv_field(&result.account_number),
        csv_field(result.check_digits.as_deref().unwrap_or("")),
        csv_field(&result.formatted),
        csv_field(&result.raw),
        csv_field(result.iban.as_deref().unwrap_or("")),
        result.valid,
    )
}

/// Write bank account CSV header + rows to any [`Write`] destination.
pub fn write_account_csv<W: Write>(
    writer: &mut W,
    results: &[AccountResult],
) -> std::io::Result<()> {
    writeln!(writer, "{}", ACCOUNT_HEADER)?;
    for result in results {
        writeln!(writer, "{}", account_row(result))?;
    }
    Ok(())
}

/// CSV header for driver's license rows.
pub const LICENSE_HEADER: &str = "country,country_name,name,code,state,valid";

/// Format a single driver's license result as a CSV row.
pub fn license_row(result: &DriverLicenseResult) -> String {
    format!(
        "{},{},{},{},{},{}",
        csv_field(&result.country_code),
        csv_field(&result.country_name),
        csv_field(&result.name),
        csv_field(&result.code),
        csv_field(result.state.as_deref().unwrap_or("")),
        result.valid
    )
}

/// CSV header for tax ID rows.
pub const TAX_HEADER: &str = "country,country_name,name,code,holder_type,valid";

/// Format a single tax ID result as a CSV row.
pub fn tax_row(result: &TaxIdResult) -> String {
    format!(
        "{},{},{},{},{},{}",
        csv_field(&result.country_code),
        csv_field(&result.country_name),
        csv_field(&result.name),
        csv_field(&result.code),
        csv_field(result.holder_type.as_deref().unwrap_or("")),
        result.valid
    )
}

/// CSV header for passport rows.
pub const PASSPORT_HEADER: &str = "country,country_name,name,code,valid";

/// Format a single passport result as a CSV row.
pub fn passport_row(result: &PassportResult) -> String {
    format!(
        "{},{},{},{},{}",
        csv_field(&result.country_code),
        csv_field(&result.country_name),
        csv_field(&result.name),
        csv_field(&result.code),
        result.valid
    )
}
