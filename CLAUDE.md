# Disc-over — working notes

Personal music-library app on Discogs data. Tauri v2 + Rust behind React +
TypeScript, for iPhone and a macOS/Linux Companion.

- `docs/disc-over-spec.md` is the specification. Read it before any change and
  check finished work against it. `docs/build-status.md` tracks the phases.
- All rules live in `crates/discover-core` (no Tauri dependency).
  `src-tauri/src/commands.rs` stays thin, so the phone and the Companion
  cannot disagree.
- Conventions shared with Archiva: UUID v7 text ids, minted once and never
  derived; UTC `YYYY-MM-DD HH:MM:SS` dates; optional extras in `attribute`.
- Migrations are append-only. Never edit one that has shipped.
- Every user-facing write goes through `discover-core::library`, which appends to
  `change_log` for sync.
- Before reporting anything as working: `cargo test`, `npm test` and
  `npm run build` all pass.
- Sign-in tokens go in the Keychain, never in the database, CSV or export.
