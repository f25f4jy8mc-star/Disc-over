//! The Discogs grading scale, best first: M, NM, VG+, VG, G+, G, F, P.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Grade {
    #[serde(rename = "M")]
    Mint,
    #[serde(rename = "NM")]
    NearMint,
    #[serde(rename = "VG+")]
    VeryGoodPlus,
    #[serde(rename = "VG")]
    VeryGood,
    #[serde(rename = "G+")]
    GoodPlus,
    #[serde(rename = "G")]
    Good,
    #[serde(rename = "F")]
    Fair,
    #[serde(rename = "P")]
    Poor,
}

impl Grade {
    pub const ALL: [Grade; 8] = [
        Grade::Mint,
        Grade::NearMint,
        Grade::VeryGoodPlus,
        Grade::VeryGood,
        Grade::GoodPlus,
        Grade::Good,
        Grade::Fair,
        Grade::Poor,
    ];

    /// The short code stored in the database and shown on screen.
    pub fn code(self) -> &'static str {
        match self {
            Grade::Mint => "M",
            Grade::NearMint => "NM",
            Grade::VeryGoodPlus => "VG+",
            Grade::VeryGood => "VG",
            Grade::GoodPlus => "G+",
            Grade::Good => "G",
            Grade::Fair => "F",
            Grade::Poor => "P",
        }
    }

    pub fn from_code(s: &str) -> Option<Grade> {
        Grade::ALL.into_iter().find(|g| g.code() == s.trim())
    }

    /// Discogs' API spells grades out in full, e.g. "Very Good Plus (VG+)".
    /// The code in brackets is the part that matters.
    pub fn from_discogs(s: &str) -> Option<Grade> {
        let inner = s.rsplit_once('(').and_then(|(_, r)| r.strip_suffix(')'));
        Grade::from_code(inner.unwrap_or(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip() {
        for g in Grade::ALL {
            assert_eq!(Grade::from_code(g.code()), Some(g));
        }
    }

    #[test]
    fn reads_discogs_spelling() {
        assert_eq!(Grade::from_discogs("Very Good Plus (VG+)"), Some(Grade::VeryGoodPlus));
        assert_eq!(Grade::from_discogs("Mint (M)"), Some(Grade::Mint));
        assert_eq!(Grade::from_discogs("Generic"), None);
    }

    #[test]
    fn best_sorts_first() {
        assert!(Grade::Mint < Grade::GoodPlus);
    }
}
