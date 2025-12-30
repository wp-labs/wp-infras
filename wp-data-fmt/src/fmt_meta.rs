use chrono::NaiveDateTime;

pub fn date_from(s: &str) -> Option<NaiveDateTime> {
    // naive parser; keep parity with original helper
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_from_valid() {
        let result = date_from("2024-01-15 10:30:45");
        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
        assert_eq!(dt.hour(), 10);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_date_from_boundary_values() {
        // Start of year
        let result = date_from("2024-01-01 00:00:00");
        assert!(result.is_some());

        // End of year
        let result = date_from("2024-12-31 23:59:59");
        assert!(result.is_some());
    }

    #[test]
    fn test_date_from_invalid_format() {
        // Wrong separator
        assert!(date_from("2024/01/15 10:30:45").is_none());
        // Missing time
        assert!(date_from("2024-01-15").is_none());
        // ISO format with T
        assert!(date_from("2024-01-15T10:30:45").is_none());
        // Empty string
        assert!(date_from("").is_none());
        // Invalid date
        assert!(date_from("2024-13-45 10:30:45").is_none());
    }

    use chrono::Datelike;
    use chrono::Timelike;
}
