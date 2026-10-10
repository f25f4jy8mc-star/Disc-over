# Disc-over: App Specification

10 October 2026

## Overview

Disc-over is a personal iPhone app that catalogs a physical music library (vinyl, CDs, cassettes) using Discogs data. It shows Discogs pricing per release, plays anything you search for through Spotify or YouTube, and tracks which albums you also own as digital files.

It has three pieces:

| Piece | Runs on | Job |
| --- | --- | --- |
| Disc-over app | iPhone | Catalog, search, playback, pricing, wishlist, stats |
| Disc-over Companion | Mac or Linux | Holds the main library, matches digital files, writes tags, syncs with the phone over USB, exports CSV |
| Archiva export | Companion | Writes chosen parts of the library into a folder that Archiva indexes |

The app is for one user (you). There are no accounts, no server and no sharing features in this version.

## Features

### Adding items

| Method | How it works | Phase |
| --- | --- | --- |
| Discogs search | Type artist, title or catalog number; pick the exact pressing | 1 |
| Barcode scan | Camera reads the barcode; app queries Discogs by barcode and lists matching pressings | 1 |
| Discogs collection import | Sign in to Discogs once; pull every release in your collection, including folders, your grading and notes | 1 |
| Cover photo | Photograph the sleeve; app reads the text on it and runs a Discogs search with the result | 3 |

One barcode often matches several pressings, so every method ends on a "pick the pressing" screen before saving.

### What is stored per release

Artist, title, year, label, catalog number, format (vinyl, CD or cassette, plus details such as LP, 7-inch, reissue), country, genres and styles, tracklist, cover image, barcode, Discogs release ID and master ID.

### What is stored per copy you own

- Media condition and sleeve condition, on the Discogs grading scale (M, NM, VG+, VG, G+, G, F, P)
- Price paid, currency and purchase date
- Storage location (free text, with suggestions from locations already used)
- Personal notes
- Your own photos of the copy
- Review: a rating from 1 to 5 and review text

Owning two copies of one release is supported; each copy has its own record.

### Wishlist

Any Discogs release can be added to the wishlist from search. Wishlist items show the same pricing as owned items, and can be imported from your Discogs wantlist.

### Stats

Total items, total estimated value (low, median, high), total spent, counts by format, decade, genre and label, and the ten most valuable items.

### Filters

Filter the library by format, genre, decade, label, condition, storage location and "owned digitally". Sort by artist, title, date added, year and value.

## Search and playback

Search is the home screen of the app. One search bar returns two separate sections:

1. **In my library**: owned copies and wishlist items, searched on the phone with no network needed. Matches artist, title, track names, label, catalog number, notes and storage location.
2. **On Discogs**: live results from the Discogs database, each marked Owned, Wishlist or neither.

Results can be albums or individual tracks. Tapping an album opens its page with the tracklist; every album and every track has a play button, whether you own it or not.

### Playback rules

| Order | Source | When it is used | What happens |
| --- | --- | --- | --- |
| 1 | Spotify | A matching album or track is found on Spotify | The app tells the Spotify app to play it; controls stay inside Disc-over |
| 2 | YouTube | No Spotify match, and the Discogs release lists videos | An embedded YouTube player opens inside Disc-over |
| 3 | Link out | Neither is available | Buttons open Apple Music, YouTube Music or Bandcamp search |

Spotify requirements: a Spotify Premium account, the Spotify app installed on the iPhone, and a one-time sign-in from Disc-over. Spotify's iOS SDK controls the Spotify app in the background; it does not stream audio into Disc-over itself.

### Matching a Discogs release to Spotify

1. Search Spotify by barcode (UPC).
2. If nothing matches, search by artist and album title, then compare track count and year.
3. Save the match on the release so it is only looked up once. A wrong match can be corrected by hand.

Different pressings of one album share a single Spotify album, so the saved match is stored against the Discogs master ID as well as the release.

### Mini player

A bar at the bottom of every screen shows the current track with play, pause, next and previous.

## Pricing

Each release shows three figures in the style of the Discogs site: low, median and high. The whole library shows the same three as totals.

| Figure | Discogs API source | Notes |
| --- | --- | --- |
| Collection total (low, median, high) | Collection value endpoint | Covers your whole Discogs collection in one call; needs the library synced to Discogs |
| Suggested price by condition | Price suggestions endpoint | One price per grade, so the app can show the price for your copy's actual condition |
| Lowest current listing and number for sale | Release and marketplace stats | Live marketplace figure |
| Per-release low, median, high from past sales | Not in the API | Shown on the Discogs website only; the developer documentation lists no endpoint for it |

Checked against the Discogs developer documentation on 10 October 2026: the release data includes the lowest listing price and number for sale, and the documentation lists Price Suggestions and Collection Value sections. Their exact response fields were not readable on the page and are confirmed by a test call in phase 0.

Per-release low, median and high are therefore built from suggested prices:

- Low = suggested price for grade G+
- Median = suggested price for your copy's grade (VG+ when no copy is owned)
- High = suggested price for grade M

### Refresh rules

- Prices refresh when a release page is opened, if the saved figure is more than 24 hours old.
- A "Refresh all prices" action updates the whole library in the background, paced to stay inside the Discogs rate limit (60 requests a minute when signed in).
- Every refresh saves a dated snapshot, so value history can be added later without new data collection.
- Currency follows the setting on your Discogs account.

## Data model

The phone and the Companion each hold a SQLite database with the same tables. Three conventions are copied from Archiva so the two projects stay compatible:

- Every record ID is a UUID v7 stored as text. IDs are created once and never change, whatever happens to the title, the Discogs match or the file path.
- Dates are stored as text in `YYYY-MM-DD HH:MM:SS` form, in UTC.
- Optional extras go in a key/value table, so adding a field never needs a database migration.

| Table | One row per | Main fields |
| --- | --- | --- |
| `release` | Discogs release | id, discogs_release_id, discogs_master_id, artist, title, year, label, catalog_no, format (vinyl, cd, cassette), format_detail, country, barcode, cover_image_path |
| `track` | Track on a release | id, release_id, position, title, duration_seconds |
| `release_tag` | Genre or style on a release | release_id, kind (genre, style), name |
| `copy` | Physical copy you own | id, release_id, media_condition, sleeve_condition, price_paid, currency, purchased_on, location, notes, rating, review, added_at |
| `copy_photo` | Your photo of a copy | id, copy_id, file_path, taken_at |
| `wishlist_item` | Wanted release | id, release_id, added_at, notes |
| `price_snapshot` | Price check | id, release_id, low, median, high, lowest_listing, num_for_sale, currency, checked_at |
| `streaming_link` | Link to a service | release_id, track_id (optional), service (spotify, apple_music, youtube, youtube_music, bandcamp), url, confirmed |
| `digital_album` | Matched folder of files | id, release_id, folder_path, match_state (suggested, confirmed, rejected) |
| `digital_file` | Audio file | id, digital_album_id, track_id, file_path, format, tags_written_at |
| `attribute` | Extra fact | owner_id, key, value, value_num |
| `change_log` | Edit awaiting sync | id, table_name, row_id, field, new_value, changed_at, device |

The split between `release` and `copy` matters: pricing, tracks and streaming links belong to the release, while condition, price paid, location and review belong to the copy.

## Desktop companion

The Companion is a desktop app that holds the main copy of the library. It is built on the same stack as Archiva (Tauri v2, Rust, React and TypeScript, SQLite), so code and knowledge carry between the two projects.

### Digital file matching

1. You point the Companion at the folders, drives or cloud-synced folders that hold music files.
2. It reads the existing tags of every mp3, m4a, aac, flac, aiff, wav and ogg file.
3. It groups files into albums and proposes a match to a release in the library, using artist, album title and track count.
4. You confirm or reject each proposed match. Nothing is written until a match is confirmed.

Confirmed matches set the "owned digitally" flag that the phone shows. Library releases with no match appear in a "Not ripped yet" list.

### Tag writing

For confirmed matches, the Companion writes Discogs data into the files:

| Tag | Value |
| --- | --- |
| Album, album artist, artist, track title | From the Discogs release |
| Track number, disc number | From the Discogs tracklist position |
| Year, genre, label, catalog number | From the Discogs release |
| `DISCOGS_RELEASE_ID` | The release ID, so the match survives renaming |
| Cover art | Optional, off by default |

Rules for writing:

- A preview shows every change, old value beside new, before anything is written.
- Files are edited in place. The Companion never renames or moves a music file.
- The original tags are saved in the Companion's database, so any write can be undone.

### USB sync with the phone

The Companion's database is the main library; the phone holds a working copy.

1. The phone app saves its database and photos in its Documents folder, with iOS file sharing switched on.
2. With the phone plugged in, the Companion reads that folder over USB.
3. Each side sends the rows in its `change_log` that the other has not seen.
4. If both sides changed the same field, the later edit wins, and the conflict is listed for review.
5. The Companion writes the merged database back to the phone.

Reading an app's Documents folder over USB from a third-party desktop app relies on the open-source libimobiledevice library, which runs on both macOS and Linux. If that proves unreliable, the fallback is sync over the local network, using the same change log.

### CSV export

One row per copy, with every release and copy field as a column, plus the latest low, median and high price. The file opens in Excel or Numbers and uses UTF-8 text.

## Archiva compatibility

Disc-over connects to Archiva by writing a folder of Markdown notes and images that Archiva indexes as a linked folder. This is the only route that works with Archiva as it is built today: Archiva has no import command and no network interface, and it takes in content only by scanning folders.

### How Archiva's backend works

These points come from the Archiva code supplied on 10 October 2026 (`src-tauri/src/model/` and `migrations_model/`).

| Area | What Archiva does | Where in the code |
| --- | --- | --- |
| Stack | Tauri v2, Rust backend, React and TypeScript front end, SQLite through rusqlite | `Cargo.toml` |
| Library | A Space is a folder you choose, holding `archiva.sqlite`, `archiva-space.json`, `proxies/` and `notes/` | `spaces.rs` |
| Intake | Scans every enabled linked folder (a `source` row with an absolute path) when you press Refresh | `sources.rs`, `scan.rs` |
| Skipped | Any file or folder whose name starts with a dot, and everything inside the Space folder itself | `scan.rs`, `commands.rs` |
| Items | Everything is a `node` of type media, note, collector or tag | `001_model.sql` |
| IDs | UUID v7 as text, never derived from path, name or contents | `001_model.sql`, `scan.rs` |
| Notes | A `.md` or `.markdown` file becomes a note; its ID lives in the file as `archiva-id` in the frontmatter | `content_type.rs`, `signals.rs` |
| Folders | Each folder becomes a collector, and each file gets a `contains` link from its folder | `folders.rs` |
| Tags | Six fixed facets: format, era (tier 1), environment, action (tier 2), attribute, subject (tier 3) | `facets.rs`, `tags.rs` |
| Extra facts | `attribute` table of key, text value and numeric value per node | `001_model.sql`, `extract.rs` |
| Search | Full-text index over item name, note body (frontmatter removed) and tags | `001_model.sql`, `scan.rs` |
| Web items | A node can be a `remote_url` holding an http or https address | `identity.rs` |

File types Archiva indexes: jpg, jpeg, png, tif, tiff, heic, heif, webp, gif, bmp, svg, psd, mp4, m4v, mov, mkv, avi, webm, mp3, m4a, aac, flac, aif, aiff, wav, ogg, pdf, obj, fbx, stl, gltf, glb, md, markdown. Every other type, including `.json` and `.csv`, is ignored.

### Export contract

The Companion has an "Export to Archiva" screen where you choose what to send and how to group it.

**What can be exported (tick any):** owned copies, wishlist, reviews and notes, your photos, cover images, pricing, digital-file matches.

**Grouping (choose one):** by format, by artist, by decade, by genre or by storage location. The grouping becomes the folder layout, and Archiva turns each folder into a collector.

The Companion must follow these rules so Archiva reads the export correctly:

1. **Location.** The export folder is outside every Archiva Space folder. Archiva never indexes its own Space, so an export placed inside one is invisible.
2. **Linking.** You add the export folder to Archiva once as a linked folder. After each export, pressing Refresh in Archiva picks up the changes.
3. **File types.** One `.md` note per copy or wishlist item, plus `.jpg` or `.png` images. No other file types.
4. **No dot names.** No exported file or folder name starts with a dot. Temporary files do start with a dot, so Archiva skips them.
5. **Safe writes.** Each file is written to a dot-prefixed temporary name and then renamed into place. Archiva leaves alone any file that is empty or was modified moments ago.
6. **Text format.** UTF-8 with no byte-order mark, and LF line endings. The first line of every note is exactly `---`.
7. **Flat frontmatter.** One `key: value` per line, with no nesting and no multi-line values. Archiva reads frontmatter line by line, without a YAML parser.
8. **IDs.** `archiva-id` holds the record's own UUID v7 from the Disc-over database: the `copy` ID or the `wishlist_item` ID. Archiva adopts an ID it has not seen before (rule 5 of its reconciliation ladder), so both apps share one ID per item.
9. **One ID, one file.** No two files ever carry the same `archiva-id`. If two do, Archiva gives the second a fresh ID and rewrites that file.
10. **Re-export.** An existing note is updated in place and keeps its `archiva-id` line. A file whose contents have not changed is not rewritten.
11. **File names.** `Artist - Title (Year) [Format].md`, with the characters `/ \ : * ? " < > |` replaced by `-`, at most 120 characters, and unique across the whole export. Archiva shows the file name, minus extension, as the item's name.
12. **Images.** Named after their note: `Artist - Title (Year) [Format] cover.jpg` and `... photo 1.jpg`. Images cannot carry an ID, so Archiva tracks them by file identity, then contents; unchanged images are never rewritten.
13. **Searchable body.** Every fact worth finding in Archiva appears as plain text in the note body, because Archiva's search indexes the body and not the frontmatter.
14. **Removal.** When a record is deleted in Disc-over, its files are removed from the export. Archiva then marks the item as missing; it never deletes it or the tags and links you added.

### Example note

```markdown
---
archiva-id: 0199c2f4-7a10-7c3e-9b1a-4f2d8e6a1b05
title: Miles Davis - Kind of Blue (1959) [Vinyl]
discover-schema: 1
discover-kind: copy
discover-release-id: 0199c2f4-6e02-7b11-8c3d-2a9f1d4c7e90
discogs-release-id: 1234567
discogs-master-id: 5460
discover-tag-format: Vinyl
discover-tag-era: 1950s
discover-tag-subject: Miles Davis
discover-tag-attribute: Jazz; Modal
discover-price-low: 18.00
discover-price-median: 32.50
discover-price-high: 75.00
discover-price-currency: GBP
discover-price-checked: 2026-10-10
---

# Miles Davis - Kind of Blue

![[Miles Davis - Kind of Blue (1959) [Vinyl] cover]]

Vinyl, LP, reissue. Columbia, CS 8163. 1959, US.

Condition: media VG+, sleeve VG. Location: Shelf 2, box A.
Paid 25.00 GBP on 2024-03-16. Value 18.00 / 32.50 / 75.00 GBP.

## Tracks

1. So What
2. Freddie Freeloader

## Review

5 of 5. Review text here.

## Links

- Discogs: https://www.discogs.com/release/1234567
- Spotify: https://open.spotify.com/album/...
```

### Tag mapping

Archiva tags are identified by name and facet together, compared in lower case with spaces collapsed. Disc-over writes its tags with the same spelling every time so they land on one Archiva tag.

| Disc-over field | Archiva facet | Example | Note |
| --- | --- | --- | --- |
| Format | format | Vinyl, CD, Cassette | |
| Decade of release year | era | 1950s | Same label style Archiva uses for its own era suggestions |
| Artist | subject | Miles Davis | |
| Genre and style | attribute | Jazz, Modal | |
| Storage location | environment | Shelf 2 | Optional; off by default |
| (none) | action | | Unused |

### What works today, and what needs an addition to Archiva

| Feature | Status | What is needed |
| --- | --- | --- |
| Notes and images appear in Archiva | Works today | Nothing |
| Folder grouping appears as collectors | Works today | Nothing |
| Note text is searchable | Works today | Nothing |
| Items keep identity across re-exports, renames and moves | Works today | Nothing |
| Shared ID between the two apps | Works today | Nothing |
| Tags applied from the `discover-tag-*` lines | Needs addition | A new Archiva module that reads the lines and creates suggested `tag_of` links (status `suggested`, origin `extension`) for you to accept |
| Prices and Discogs IDs as sortable Archiva attributes | Needs addition | The same module writing `attribute` rows with keys prefixed `discover.`, numbers in `value_num` |
| Cover embedded in the note, links between notes | Needs addition | Archiva's planned wikilink and embed feature, listed as unbuilt in its checklist |
| Streaming links as Archiva web items | Needs addition | The same module calling Archiva's existing remote-item function |

Archiva's rule is that software suggests and you classify, so the tag import arrives as suggestions and nothing is applied without your acceptance. The module is a new file beside the existing ones; Archiva's tested modules and its first migration stay untouched, as its working notes require.

### Digital music files in Archiva

Archiva already indexes audio files and records duration, bit rate, sample rate, channels and codec. It does not read artist or album tags. Your music folders can be linked into Archiva directly, alongside the Disc-over export.

Tag writing changes a file's contents, size and modified date at once. Archiva still recognises the file because its path and file identity are unchanged, which is why the Companion edits in place and never renames.

## External services

| Service | Used for | Sign-in | Limits to design around |
| --- | --- | --- | --- |
| Discogs API | Search, barcode lookup, release data, collection and wantlist import, pricing, video links | Your Discogs account, once | 60 requests a minute signed in, 25 signed out; search and images need sign-in |
| Spotify iOS SDK | Playing albums and tracks | Your Spotify account, once | Premium needed for on-demand play; Spotify app must be installed; the SDK remote-controls the Spotify app |
| Spotify Web API | Finding the Spotify album for a release | Same sign-in | Development mode returns at most 10 search results per type; the developer account needs Premium (rules changed February 2026) |
| YouTube embedded player | Fallback playback | None | Plays only videos that allow embedding; no background audio |
| Apple Music, YouTube Music, Bandcamp | Link-out buttons | None | Search links built from artist and title; no API used |

All sign-in tokens are stored in the iPhone Keychain and never written to the database, the CSV or the Archiva export.

## Build order

Each phase ends with something usable, and later phases do not require rewriting earlier ones.

| Phase | Delivers | Main work |
| --- | --- | --- |
| 0. Checks | Answers to the open questions below | Test the Discogs price endpoints with your account; test Spotify sign-in and playback in a blank app; test reading a phone folder over USB |
| 1. Phone catalog | Search, barcode scan, Discogs import, release and copy pages, pricing | Database, Discogs sign-in, two-section search, pick-the-pressing screen |
| 2. Playback | Play buttons and mini player | Spotify matching and control, YouTube fallback, link-out buttons |
| 3. Library extras | Wishlist, stats, filters, cover-photo adding | Mostly screens over data that already exists |
| 4. Companion | Main library on the desktop, USB sync, CSV export | Tauri desktop app, change-log sync |
| 5. Digital files | Matching and tag writing | Folder scanning, match review screen, tag preview and undo |
| 6. Archiva export | Export screen and the folder contract above | Note and image writer; then the Archiva-side module for tags and attributes |

### Technology

| Piece | Recommended | Reason |
| --- | --- | --- |
| Companion | Tauri v2, Rust, React, TypeScript, SQLite | Same as Archiva |
| iPhone app | Tauri v2 for iOS with the same React code, plus one small Swift plugin for Spotify | One codebase and one language for screens across both apps |
| Alternative for iPhone | Native SwiftUI | Simpler Spotify, camera and Keychain access, but a second language and no shared screens |

Decided on 10 October 2026: Tauri v2 for both apps. The Companion is built for macOS first and for Linux from the same code; every Companion feature in this spec must work on both.

## Open questions and risks

- [x] Does the Discogs API return per-release low, median and high? No. The figures are built from suggested prices by condition.
- [x] Tauri for iOS or SwiftUI for the phone app? Tauri.
- [x] Mac or Windows? Mac and iPhone, with the Companion also running on Linux.
- [ ] Do the Price Suggestions and Collection Value calls return what the pricing section assumes? Test with your Discogs account.
- [ ] Does USB access to the app's Documents folder work reliably on macOS and Linux, or is network sync needed?
- [ ] Should wishlist items be exported to Archiva by default, or only owned copies?
- [ ] Should the Archiva-side module (tags, attributes, web items) be built as part of this project or as part of Archiva?
- [ ] Which price currency should be shown if it differs from your Discogs account setting?

Risks:

- **Spotify rule changes.** Spotify tightened developer access twice since late 2024. YouTube and link-out playback keep the feature working if Spotify access is lost.
- **Wrong streaming match.** Barcode and title matching will sometimes pick the wrong edition; every match can be corrected by hand.
- **Cover-photo recognition.** Text on sleeves is often stylised or absent; this method is scheduled last and may stay a convenience, not a main route.
- **Archiva is still changing.** The export contract depends on Archiva's scan and frontmatter rules as of 10 October 2026. The `discover-schema` line lets a later Archiva tell old exports from new ones.

## Sources

- [Spotify iOS SDK](https://github.com/spotify/ios-sdk)
- [Notes on Spotify Web API restrictions and the February 2026 changes](https://pypi.org/project/mcp-spotify/0.1.0/)
- [spotify-cli setup notes on development mode](https://github.com/tszaks/spotify-cli)
- Archiva source code, supplied 10 October 2026
- [Discogs API documentation](https://www.discogs.com/developers), opened 10 October 2026
