//! Record ids: UUID v7 as text, minted once and never changed, whatever
//! happens to the title, the Discogs match or the file path. The same rule as
//! Archiva, which is what lets an exported note carry its `copy` id into
//! Archiva as `archiva-id`.

pub fn new_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_v7_and_distinct() {
        let a = new_id();
        let b = new_id();
        assert_ne!(a, b);
        let parsed = uuid::Uuid::parse_str(&a).unwrap();
        assert_eq!(parsed.get_version_num(), 7);
    }
}
