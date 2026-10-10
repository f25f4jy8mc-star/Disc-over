//! Naming rules from the Archiva export contract (spec: "Export contract").
//! The export writer itself is phase 6; these rules live here now because the
//! tag spelling and file naming must be stable from the first export onward.

use std::collections::HashSet;

/// Rule 11 caps file names at 120 characters. Images take the note's name plus
/// a suffix (" cover.jpg", " photo 12.jpg"), so the shared stem is capped
/// lower to keep every derived file inside the limit.
pub const MAX_STEM_CHARS: usize = 100;

const FORBIDDEN: [char; 9] = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

/// Display spelling of a stored format, used in names and as the `format` tag.
pub fn format_label(format: &str) -> &str {
    match format {
        "vinyl" => "Vinyl",
        "cd" => "CD",
        "cassette" => "Cassette",
        other => other,
    }
}

/// The `era` tag: decade of the release year, e.g. 1959 -> "1950s".
pub fn decade_label(year: i32) -> String {
    format!("{}s", year - year.rem_euclid(10))
}

/// `Artist - Title (Year) [Format]`, forbidden characters replaced by `-`,
/// no leading dot (rule 4 — Archiva skips dot names), capped in length.
pub fn note_stem(artist: &str, title: &str, year: Option<i32>, format: &str) -> String {
    let year = year.map(|y| format!(" ({y})")).unwrap_or_default();
    let raw = format!("{artist} - {title}{year} [{}]", format_label(format));
    let cleaned: String = raw
        .chars()
        .map(|c| if FORBIDDEN.contains(&c) || c.is_control() { '-' } else { c })
        .collect();
    let cleaned = cleaned.trim_start_matches('.').trim();
    truncate_chars(cleaned, MAX_STEM_CHARS).trim_end().to_string()
}

fn truncate_chars(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

/// Makes stems unique across one export (rule 11) by appending " 2", " 3"…
/// Comparison is case-insensitive, because macOS file systems usually are.
#[derive(Default)]
pub struct StemAllocator {
    taken: HashSet<String>,
}

impl StemAllocator {
    pub fn allocate(&mut self, stem: &str) -> String {
        let mut n = 1;
        loop {
            let candidate = if n == 1 {
                stem.to_string()
            } else {
                let suffix = format!(" {n}");
                let room = MAX_STEM_CHARS - suffix.chars().count();
                format!("{}{suffix}", truncate_chars(stem, room).trim_end())
            };
            if self.taken.insert(candidate.to_lowercase()) {
                return candidate;
            }
            n += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stem_matches_spec_example() {
        assert_eq!(
            note_stem("Miles Davis", "Kind of Blue", Some(1959), "vinyl"),
            "Miles Davis - Kind of Blue (1959) [Vinyl]"
        );
    }

    #[test]
    fn forbidden_characters_and_dots() {
        assert_eq!(
            note_stem("AC/DC", "Who Made Who?", None, "cd"),
            "AC-DC - Who Made Who- [CD]"
        );
        assert!(!note_stem("...And You Will Know Us", "X", None, "cd").starts_with('.'));
    }

    #[test]
    fn long_names_fit_with_image_suffixes() {
        let stem = note_stem(&"é".repeat(300), "T", Some(2000), "vinyl");
        assert!(stem.chars().count() <= MAX_STEM_CHARS);
        assert!(format!("{stem} photo 12.jpg").chars().count() <= 120);
    }

    #[test]
    fn duplicates_are_numbered() {
        let mut a = StemAllocator::default();
        assert_eq!(a.allocate("X - Y [CD]"), "X - Y [CD]");
        assert_eq!(a.allocate("x - y [cd]"), "x - y [cd] 2");
        assert_eq!(a.allocate("X - Y [CD]"), "X - Y [CD] 3");
    }

    #[test]
    fn decades() {
        assert_eq!(decade_label(1959), "1950s");
        assert_eq!(decade_label(2000), "2000s");
    }
}
