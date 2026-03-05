use anyhow::{Result, bail};

/// Validate a date string is in YYYY-MM-DD format.
pub fn date(s: &str) -> Result<()> {
    if s.len() != 10 {
        bail!("invalid date format '{s}': expected YYYY-MM-DD");
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        bail!("invalid date format '{s}': expected YYYY-MM-DD");
    }
    let year: u16 = parts[0]
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid year in date '{s}'"))?;
    let month: u8 = parts[1]
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid month in date '{s}'"))?;
    let day: u8 = parts[2]
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid day in date '{s}'"))?;

    if year < 1900 || year > 2100 {
        bail!("year out of range in date '{s}'");
    }
    if !(1..=12).contains(&month) {
        bail!("month out of range in date '{s}'");
    }
    if !(1..=31).contains(&day) {
        bail!("day out of range in date '{s}'");
    }
    Ok(())
}

/// Validate a UUID string (basic format check).
pub fn uuid(s: &str) -> Result<()> {
    // Accept UUIDs with or without hyphens
    let hex: String = s.chars().filter(|c| *c != '-').collect();
    if hex.len() != 32 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("invalid UUID format '{s}'");
    }
    Ok(())
}

/// Validate that a JSON string parses correctly.
pub fn json(s: &str) -> Result<serde_json::Value> {
    serde_json::from_str(s).map_err(|e| anyhow::anyhow!("invalid JSON: {e}"))
}

/// Validate a file path for output (parent directory must exist).
pub fn output_path(s: &str) -> Result<()> {
    let path = std::path::Path::new(s);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            bail!("output directory '{}' does not exist", parent.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_dates() {
        assert!(date("2024-01-01").is_ok());
        assert!(date("2024-12-31").is_ok());
        assert!(date("1999-06-15").is_ok());
    }

    #[test]
    fn invalid_dates() {
        assert!(date("2024-13-01").is_err()); // month > 12
        assert!(date("2024-00-01").is_err()); // month 0
        assert!(date("2024-01-32").is_err()); // day > 31
        assert!(date("2024-01-00").is_err()); // day 0
        assert!(date("not-a-date").is_err());
        assert!(date("20240101").is_err()); // no hyphens
        assert!(date("").is_err());
    }

    #[test]
    fn valid_uuids() {
        assert!(uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(uuid("550e8400e29b41d4a716446655440000").is_ok());
    }

    #[test]
    fn invalid_uuids() {
        assert!(uuid("not-a-uuid").is_err());
        assert!(uuid("").is_err());
        assert!(uuid("550e8400-e29b-41d4-a716-44665544000").is_err()); // too short
        assert!(uuid("550e8400-e29b-41d4-a716-4466554400000").is_err()); // too long
        assert!(uuid("550e8400-e29b-41d4-a716-44665544000g").is_err()); // invalid char
    }

    #[test]
    fn valid_json() {
        assert!(json("{}").is_ok());
        assert!(json(r#"{"key": "value"}"#).is_ok());
        assert!(json("[1, 2, 3]").is_ok());
    }

    #[test]
    fn invalid_json() {
        assert!(json("not json").is_err());
        assert!(json("{missing: quotes}").is_err());
        assert!(json("").is_err());
    }

    #[test]
    fn output_path_validation() {
        assert!(output_path("output.pdf").is_ok());
        assert!(output_path("/tmp/output.pdf").is_ok());
        assert!(output_path("/nonexistent/dir/output.pdf").is_err());
    }
}
