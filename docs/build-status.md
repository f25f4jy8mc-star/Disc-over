# Build status

Phases from the spec's "Build order". Update this as work lands.

## Phase 0 — checks (open)

- [ ] Price Suggestions and Collection Value: test calls with your Discogs account
- [ ] Spotify sign-in and playback in a blank app
- [ ] Reading the app's Documents folder over USB (libimobiledevice) on macOS and Linux

## Phase 1 — phone catalog (started)

Done:

- [x] Shared schema, all twelve tables from the spec (`migrations/001_schema.sql`), versioned by `PRAGMA user_version`
- [x] UUID v7 text ids; UTC `YYYY-MM-DD HH:MM:SS` dates
- [x] Discogs grading scale, including Discogs' spelled-out form ("Very Good Plus (VG+)")
- [x] Low / median / high from suggested prices (G+, copy grade or VG+, M) and the 24-hour staleness rule
- [x] Saving a release once and any number of copies of it; wishlist items
- [x] Every write appended to `change_log`, one row per field, ready for sync
- [x] "In my library" offline search: artist, title, tracks, label, catalog number, notes, location
- [x] Search home screen, library section wired to the backend
- [x] iOS file sharing switched on (`Info.ios.plist`) for USB sync

Next:

- [ ] Discogs OAuth sign-in, token in the iPhone Keychain
- [ ] "On Discogs" search section, marked Owned / Wishlist
- [ ] Pick-the-pressing screen
- [ ] Barcode scan
- [ ] Discogs collection and wantlist import
- [ ] Release and copy pages; pricing fetch and `price_snapshot` rows

## Also in place early

- Archiva export naming (phase 6): `Artist - Title (Year) [Format]`, forbidden
  characters, no leading dot, unique names, era tag as decade. The stem is
  capped at 100 characters so `… photo 12.jpg` still fits the 120 limit.

## Decisions taken while building

- `release.discogs_release_id` is unique: one row per Discogs release, shared by
  every copy. Releases entered by hand leave it empty.
- Release data is not field-logged for sync; it is re-fetchable from Discogs,
  so only its Discogs id is logged. Copies and wishlist items are logged in full.
