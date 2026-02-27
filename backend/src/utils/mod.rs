//! Utility functions

use chrono::{DateTime, Utc};

/// Get current timestamp
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Format timestamp to ISO 8601
pub fn format_timestamp(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Parse ISO 8601 timestamp
pub fn parse_timestamp(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_roundtrip() {
        let original = now();
        let formatted = format_timestamp(&original);
        let parsed = parse_timestamp(&formatted).unwrap();
        assert_eq!(original, parsed);
    }
}
