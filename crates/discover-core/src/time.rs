//! Dates are text, `YYYY-MM-DD HH:MM:SS`, UTC — Archiva's convention. Text in
//! that form sorts and compares correctly as plain strings.

use chrono::{DateTime, NaiveDateTime, Utc};

pub const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn format(t: DateTime<Utc>) -> String {
    t.format(FORMAT).to_string()
}

pub fn now() -> String {
    format(Utc::now())
}

pub fn parse(s: &str) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(s, FORMAT)
        .ok()
        .map(|n| n.and_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let s = "2026-10-10 09:30:00";
        assert_eq!(format(parse(s).unwrap()), s);
        assert!(parse("2026-10-10T09:30:00Z").is_none());
    }
}
