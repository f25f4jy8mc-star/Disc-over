# Disc-over

A personal catalog for a physical music library — vinyl, CDs and cassettes —
built on Discogs data. It shows Discogs pricing per release, plays anything
you search for through Spotify or YouTube, and tracks which albums you also
own as digital files.

The full specification is in [`docs/disc-over-spec.md`](docs/disc-over-spec.md).

| Piece | Runs on | Where |
| --- | --- | --- |
| Disc-over app | iPhone (Tauri v2 for iOS) | `src/`, `src-tauri/` |
| Disc-over Companion | macOS, Linux (Tauri v2) | same shell, desktop build |
| Shared data model | both | `crates/discover-core/` |

## Layout

```
crates/discover-core/   the data model: schema, ids, grading, pricing, search, Archiva naming
src-tauri/              the Tauri shell; commands.rs is thin and calls discover-core
src/                    React + TypeScript screens
docs/                   the specification
```

## Develop

```sh
npm install
cargo test           # the data model; needs no webview, runs anywhere
npm test             # frontend unit tests
npm run build        # type-check and bundle the frontend
npm run tauri dev    # desktop Companion (needs the platform webview)
npm run tauri ios dev  # iPhone (macOS with Xcode)
```

On Linux the desktop build needs WebKitGTK; see the
[Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

## Status

Phase 1 groundwork. See `docs/build-status.md`.
