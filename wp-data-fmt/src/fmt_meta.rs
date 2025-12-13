use chrono::NaiveDateTime;

pub fn date_from(s: &str) -> Option<NaiveDateTime> {
    // naive parser; keep parity with original helper
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok()
}
