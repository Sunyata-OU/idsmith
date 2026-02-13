//! Optional CSV formatting for IBAN and personal ID results.
//!
//! Enable with the `csv` feature flag:
//! ```toml
//! eu-test-data-generator = { version = "...", features = ["csv"] }
//! ```

use std::io::Write;

use crate::personal_id::IdResult;

/// CSV header for IBAN rows.
pub const IBAN_HEADER: &str = "country,iban,iban_formatted,valid";

/// CSV header for personal ID rows.
pub const ID_HEADER: &str = "country,id_name,code,gender,dob,valid";

/// Format a single IBAN as a CSV row.
pub fn iban_row(iban_code: &str, formatted: &str, valid: bool) -> String {
    format!(
        "{},{},{},{}",
        &iban_code[..2],
        iban_code,
        formatted,
        valid
    )
}

/// Format a single personal ID result as a CSV row.
pub fn id_row(country: &str, id_name: &str, result: &IdResult) -> String {
    format!(
        "{},{},{},{},{},{}",
        country,
        id_name,
        result.code,
        result.gender.as_deref().unwrap_or(""),
        result.dob.as_deref().unwrap_or(""),
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
