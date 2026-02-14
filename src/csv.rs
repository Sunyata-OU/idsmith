//! Optional CSV formatting for IBAN and personal ID results.
//!
//! Enable with the `csv` feature flag:
//! ```toml
//! idsmith = { version = "...", features = ["csv"] }
//! ```

use std::io::Write;

use crate::bank_account::AccountResult;
use crate::personal_id::IdResult;

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
