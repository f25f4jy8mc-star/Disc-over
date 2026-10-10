//! Writing to and searching the library.
//!
//! Every write goes through a transaction that also appends to `change_log`,
//! one row per field set, so USB sync can replay it on the other device.
//! Spec: "USB sync with the phone".

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};

use crate::grading::Grade;
use crate::ids::new_id;
use crate::time;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewRelease {
    pub discogs_release_id: Option<i64>,
    pub discogs_master_id: Option<i64>,
    pub artist: String,
    pub title: String,
    pub year: Option<i32>,
    pub label: Option<String>,
    pub catalog_no: Option<String>,
    /// `vinyl`, `cd` or `cassette`.
    pub format: String,
    pub format_detail: Option<String>,
    pub country: Option<String>,
    pub barcode: Option<String>,
    /// (position, title, duration in seconds)
    pub tracks: Vec<(String, String, Option<i64>)>,
    pub genres: Vec<String>,
    pub styles: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewCopy {
    pub media_condition: Option<Grade>,
    pub sleeve_condition: Option<Grade>,
    pub price_paid: Option<f64>,
    pub currency: Option<String>,
    pub purchased_on: Option<String>,
    pub location: Option<String>,
    pub notes: Option<String>,
}

fn log(tx: &Transaction, device: &str, table: &str, row: &str, field: &str, value: Option<String>) -> Result<()> {
    tx.execute(
        "INSERT INTO change_log (id, table_name, row_id, field, new_value, changed_at, device)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![new_id(), table, row, field, value, time::now(), device],
    )?;
    Ok(())
}

/// Saves a release, or returns the existing id when that Discogs release is
/// already in the library — the release is shared by every copy of it.
pub fn upsert_release(conn: &mut Connection, device: &str, r: &NewRelease) -> Result<String> {
    if let Some(discogs_id) = r.discogs_release_id {
        let existing: Option<String> = conn
            .query_row(
                "SELECT id FROM release WHERE discogs_release_id = ?1",
                [discogs_id],
                |row| row.get(0),
            )
            .optional()?;
        if let Some(id) = existing {
            return Ok(id);
        }
    }

    let tx = conn.transaction()?;
    let id = new_id();
    tx.execute(
        "INSERT INTO release (id, discogs_release_id, discogs_master_id, artist, title, year,
                              label, catalog_no, format, format_detail, country, barcode)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            id, r.discogs_release_id, r.discogs_master_id, r.artist, r.title, r.year,
            r.label, r.catalog_no, r.format, r.format_detail, r.country, r.barcode
        ],
    )?;
    // Release data comes from Discogs and can be re-fetched on either device,
    // so only its identity is logged for sync.
    log(&tx, device, "release", &id, "discogs_release_id", r.discogs_release_id.map(|v| v.to_string()))?;

    for (position, title, duration) in &r.tracks {
        tx.execute(
            "INSERT INTO track (id, release_id, position, title, duration_seconds)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![new_id(), id, position, title, duration],
        )?;
    }
    for (kind, names) in [("genre", &r.genres), ("style", &r.styles)] {
        for name in names {
            tx.execute(
                "INSERT OR IGNORE INTO release_tag (release_id, kind, name) VALUES (?1, ?2, ?3)",
                params![id, kind, name],
            )?;
        }
    }
    tx.commit()?;
    Ok(id)
}

/// Adds one physical copy. Owning two copies of a release is two calls.
pub fn add_copy(conn: &mut Connection, device: &str, release_id: &str, c: &NewCopy) -> Result<String> {
    let tx = conn.transaction()?;
    let id = new_id();
    let added_at = time::now();
    let media = c.media_condition.map(Grade::code);
    let sleeve = c.sleeve_condition.map(Grade::code);
    tx.execute(
        "INSERT INTO copy (id, release_id, media_condition, sleeve_condition, price_paid,
                           currency, purchased_on, location, notes, added_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            id, release_id, media, sleeve, c.price_paid, c.currency,
            c.purchased_on, c.location, c.notes, added_at
        ],
    )?;
    let fields: [(&str, Option<String>); 9] = [
        ("release_id", Some(release_id.to_string())),
        ("media_condition", media.map(str::to_string)),
        ("sleeve_condition", sleeve.map(str::to_string)),
        ("price_paid", c.price_paid.map(|v| v.to_string())),
        ("currency", c.currency.clone()),
        ("purchased_on", c.purchased_on.clone()),
        ("location", c.location.clone()),
        ("notes", c.notes.clone()),
        ("added_at", Some(added_at.clone())),
    ];
    for (field, value) in fields {
        if value.is_some() {
            log(&tx, device, "copy", &id, field, value)?;
        }
    }
    tx.commit()?;
    Ok(id)
}

pub fn add_to_wishlist(conn: &mut Connection, device: &str, release_id: &str, notes: Option<&str>) -> Result<String> {
    let tx = conn.transaction()?;
    let id = new_id();
    let added_at = time::now();
    tx.execute(
        "INSERT INTO wishlist_item (id, release_id, added_at, notes) VALUES (?1, ?2, ?3, ?4)",
        params![id, release_id, added_at, notes],
    )?;
    log(&tx, device, "wishlist_item", &id, "release_id", Some(release_id.to_string()))?;
    log(&tx, device, "wishlist_item", &id, "added_at", Some(added_at))?;
    if let Some(n) = notes {
        log(&tx, device, "wishlist_item", &id, "notes", Some(n.to_string()))?;
    }
    tx.commit()?;
    Ok(id)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HitKind {
    Copy,
    Wishlist,
}

#[derive(Debug, Clone, Serialize)]
pub struct LibraryHit {
    pub kind: HitKind,
    /// The `copy` or `wishlist_item` id.
    pub id: String,
    pub release_id: String,
    pub artist: String,
    pub title: String,
    pub year: Option<i32>,
    pub format: String,
}

fn like_pattern(query: &str) -> String {
    let escaped = query
        .trim()
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    format!("%{escaped}%")
}

/// The "In my library" half of the home-screen search: offline, over owned
/// copies and wishlist items. Matches artist, title, track names, label,
/// catalog number, notes and storage location.
pub fn search_local(conn: &Connection, query: &str) -> Result<Vec<LibraryHit>> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    let release_match = "
        (r.artist LIKE ?1 ESCAPE '\\' OR r.title LIKE ?1 ESCAPE '\\'
         OR r.label LIKE ?1 ESCAPE '\\' OR r.catalog_no LIKE ?1 ESCAPE '\\'
         OR EXISTS (SELECT 1 FROM track t WHERE t.release_id = r.id AND t.title LIKE ?1 ESCAPE '\\'))";
    let sql = format!(
        "SELECT 'copy', c.id, r.id, r.artist, r.title, r.year, r.format
           FROM copy c JOIN release r ON r.id = c.release_id
          WHERE {release_match} OR c.notes LIKE ?1 ESCAPE '\\' OR c.location LIKE ?1 ESCAPE '\\'
         UNION ALL
         SELECT 'wishlist', w.id, r.id, r.artist, r.title, r.year, r.format
           FROM wishlist_item w JOIN release r ON r.id = w.release_id
          WHERE {release_match} OR w.notes LIKE ?1 ESCAPE '\\'
         ORDER BY 4 COLLATE NOCASE, 5 COLLATE NOCASE"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([like_pattern(query)], |row| {
        let kind: String = row.get(0)?;
        Ok(LibraryHit {
            kind: if kind == "copy" { HitKind::Copy } else { HitKind::Wishlist },
            id: row.get(1)?,
            release_id: row.get(2)?,
            artist: row.get(3)?,
            title: row.get(4)?,
            year: row.get(5)?,
            format: row.get(6)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open_in_memory;

    fn kind_of_blue() -> NewRelease {
        NewRelease {
            discogs_release_id: Some(1234567),
            discogs_master_id: Some(5460),
            artist: "Miles Davis".into(),
            title: "Kind of Blue".into(),
            year: Some(1959),
            label: Some("Columbia".into()),
            catalog_no: Some("CS 8163".into()),
            format: "vinyl".into(),
            tracks: vec![
                ("A1".into(), "So What".into(), Some(562)),
                ("A2".into(), "Freddie Freeloader".into(), Some(586)),
            ],
            genres: vec!["Jazz".into()],
            styles: vec!["Modal".into()],
            ..Default::default()
        }
    }

    #[test]
    fn same_discogs_release_is_saved_once() {
        let mut conn = open_in_memory().unwrap();
        let a = upsert_release(&mut conn, "test", &kind_of_blue()).unwrap();
        let b = upsert_release(&mut conn, "test", &kind_of_blue()).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn two_copies_of_one_release() {
        let mut conn = open_in_memory().unwrap();
        let r = upsert_release(&mut conn, "test", &kind_of_blue()).unwrap();
        let c1 = add_copy(&mut conn, "test", &r, &NewCopy::default()).unwrap();
        let c2 = add_copy(&mut conn, "test", &r, &NewCopy::default()).unwrap();
        assert_ne!(c1, c2);
        let n: i64 = conn
            .query_row("SELECT count(*) FROM copy WHERE release_id = ?1", [&r], |x| x.get(0))
            .unwrap();
        assert_eq!(n, 2);
    }

    #[test]
    fn writes_are_logged_for_sync() {
        let mut conn = open_in_memory().unwrap();
        let r = upsert_release(&mut conn, "phone", &kind_of_blue()).unwrap();
        let copy = NewCopy {
            media_condition: Some(Grade::VeryGoodPlus),
            location: Some("Shelf 2, box A".into()),
            ..Default::default()
        };
        let c = add_copy(&mut conn, "phone", &r, &copy).unwrap();
        let logged: String = conn
            .query_row(
                "SELECT new_value FROM change_log WHERE row_id = ?1 AND field = 'media_condition'",
                [&c],
                |x| x.get(0),
            )
            .unwrap();
        assert_eq!(logged, "VG+");
    }

    #[test]
    fn local_search_covers_tracks_location_and_wishlist() {
        let mut conn = open_in_memory().unwrap();
        let r = upsert_release(&mut conn, "test", &kind_of_blue()).unwrap();
        let copy = NewCopy { location: Some("Shelf 2, box A".into()), ..Default::default() };
        add_copy(&mut conn, "test", &r, &copy).unwrap();
        add_to_wishlist(&mut conn, "test", &r, None).unwrap();

        let by_track = search_local(&conn, "freddie").unwrap();
        assert_eq!(by_track.len(), 2);
        assert!(by_track.iter().any(|h| h.kind == HitKind::Wishlist));

        let by_location = search_local(&conn, "box a").unwrap();
        assert_eq!(by_location.len(), 1);
        assert_eq!(by_location[0].kind, HitKind::Copy);

        assert!(search_local(&conn, "100%").unwrap().is_empty());
        assert!(search_local(&conn, "  ").unwrap().is_empty());
    }
}
