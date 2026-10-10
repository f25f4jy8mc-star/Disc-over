//! The Tauri surface. Kept thin: every rule lives in `discover-core`, so the
//! phone and the Companion cannot drift apart.

use discover_core::library::{self, LibraryHit, NewCopy, NewRelease};
use tauri::State;

use crate::{Device, Library};

type CmdResult<T> = Result<T, String>;

fn err(e: anyhow::Error) -> String {
    format!("{e:#}")
}

#[tauri::command]
pub fn search_library(lib: State<Library>, query: String) -> CmdResult<Vec<LibraryHit>> {
    let conn = lib.0.lock().map_err(|e| e.to_string())?;
    library::search_local(&conn, &query).map_err(err)
}

/// Ends the "pick the pressing" flow: saves the release (once) and a copy.
#[tauri::command]
pub fn add_copy(
    lib: State<Library>,
    device: State<Device>,
    release: NewRelease,
    copy: NewCopy,
) -> CmdResult<String> {
    let mut conn = lib.0.lock().map_err(|e| e.to_string())?;
    let release_id = library::upsert_release(&mut conn, device.0, &release).map_err(err)?;
    library::add_copy(&mut conn, device.0, &release_id, &copy).map_err(err)
}

#[tauri::command]
pub fn add_to_wishlist(
    lib: State<Library>,
    device: State<Device>,
    release: NewRelease,
    notes: Option<String>,
) -> CmdResult<String> {
    let mut conn = lib.0.lock().map_err(|e| e.to_string())?;
    let release_id = library::upsert_release(&mut conn, device.0, &release).map_err(err)?;
    library::add_to_wishlist(&mut conn, device.0, &release_id, notes.as_deref()).map_err(err)
}
