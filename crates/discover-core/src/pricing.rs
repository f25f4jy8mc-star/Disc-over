//! Per-release low / median / high. Spec: "Pricing".
//!
//! The Discogs API has no per-release sales history, so the three figures are
//! built from the price-suggestion endpoint (one price per grade):
//!   low    = suggested price for G+
//!   median = suggested price for your copy's grade (VG+ when no copy is owned)
//!   high   = suggested price for M

use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use crate::grading::Grade;

/// A saved figure older than this is refreshed when the release page opens.
pub const STALE_AFTER_HOURS: i64 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Range {
    pub low: Option<f64>,
    pub median: Option<f64>,
    pub high: Option<f64>,
}

pub fn range_from_suggestions(suggested: &HashMap<Grade, f64>, copy_grade: Option<Grade>) -> Range {
    let median_grade = copy_grade.unwrap_or(Grade::VeryGoodPlus);
    Range {
        low: suggested.get(&Grade::GoodPlus).copied(),
        median: suggested.get(&median_grade).copied(),
        high: suggested.get(&Grade::Mint).copied(),
    }
}

/// True when `checked_at` (stored text) is missing, unreadable or over 24 h old.
pub fn is_stale(checked_at: Option<&str>, now: DateTime<Utc>) -> bool {
    match checked_at.and_then(crate::time::parse) {
        Some(t) => now - t > Duration::hours(STALE_AFTER_HOURS),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn suggestions() -> HashMap<Grade, f64> {
        HashMap::from([
            (Grade::Mint, 75.0),
            (Grade::NearMint, 50.0),
            (Grade::VeryGoodPlus, 32.5),
            (Grade::VeryGood, 22.0),
            (Grade::GoodPlus, 18.0),
        ])
    }

    #[test]
    fn median_defaults_to_vg_plus() {
        let r = range_from_suggestions(&suggestions(), None);
        assert_eq!(r, Range { low: Some(18.0), median: Some(32.5), high: Some(75.0) });
    }

    #[test]
    fn median_follows_copy_grade() {
        let r = range_from_suggestions(&suggestions(), Some(Grade::VeryGood));
        assert_eq!(r.median, Some(22.0));
    }

    #[test]
    fn staleness() {
        let now = crate::time::parse("2026-10-10 12:00:00").unwrap();
        assert!(!is_stale(Some("2026-10-10 00:00:00"), now));
        assert!(is_stale(Some("2026-10-09 11:59:59"), now));
        assert!(is_stale(None, now));
    }
}
