-- Disc-over schema, version 1. Spec: docs/disc-over-spec.md, "Data model".
--
-- Shared by the phone and the Companion. Conventions copied from Archiva:
--   * every id is a UUID v7 stored as text, minted once and never changed;
--   * every date is text, 'YYYY-MM-DD HH:MM:SS', UTC;
--   * optional extras go in `attribute`, so a new field needs no migration.
--
-- The release/copy split is deliberate: pricing, tracks and streaming links
-- belong to the release; condition, price paid, location and review belong
-- to the copy.

CREATE TABLE release (
    id                 TEXT PRIMARY KEY,
    discogs_release_id INTEGER UNIQUE,
    discogs_master_id  INTEGER,
    artist             TEXT NOT NULL,
    title              TEXT NOT NULL,
    year               INTEGER,
    label              TEXT,
    catalog_no         TEXT,
    format             TEXT NOT NULL CHECK (format IN ('vinyl', 'cd', 'cassette')),
    format_detail      TEXT,
    country            TEXT,
    barcode            TEXT,
    cover_image_path   TEXT
);
CREATE INDEX release_master ON release (discogs_master_id);
CREATE INDEX release_barcode ON release (barcode);

CREATE TABLE track (
    id               TEXT PRIMARY KEY,
    release_id       TEXT NOT NULL REFERENCES release (id) ON DELETE CASCADE,
    position         TEXT NOT NULL,
    title            TEXT NOT NULL,
    duration_seconds INTEGER
);
CREATE INDEX track_release ON track (release_id);

CREATE TABLE release_tag (
    release_id TEXT NOT NULL REFERENCES release (id) ON DELETE CASCADE,
    kind       TEXT NOT NULL CHECK (kind IN ('genre', 'style')),
    name       TEXT NOT NULL,
    PRIMARY KEY (release_id, kind, name)
);

-- Grades follow the Discogs scale.
CREATE TABLE copy (
    id               TEXT PRIMARY KEY,
    release_id       TEXT NOT NULL REFERENCES release (id),
    media_condition  TEXT CHECK (media_condition  IN ('M','NM','VG+','VG','G+','G','F','P')),
    sleeve_condition TEXT CHECK (sleeve_condition IN ('M','NM','VG+','VG','G+','G','F','P')),
    price_paid       REAL,
    currency         TEXT,
    purchased_on     TEXT,
    location         TEXT,
    notes            TEXT,
    rating           INTEGER CHECK (rating BETWEEN 1 AND 5),
    review           TEXT,
    added_at         TEXT NOT NULL
);
CREATE INDEX copy_release ON copy (release_id);

CREATE TABLE copy_photo (
    id        TEXT PRIMARY KEY,
    copy_id   TEXT NOT NULL REFERENCES copy (id) ON DELETE CASCADE,
    file_path TEXT NOT NULL,
    taken_at  TEXT
);

CREATE TABLE wishlist_item (
    id         TEXT PRIMARY KEY,
    release_id TEXT NOT NULL REFERENCES release (id),
    added_at   TEXT NOT NULL,
    notes      TEXT
);

-- Every refresh adds a row, so value history needs no new collection later.
CREATE TABLE price_snapshot (
    id             TEXT PRIMARY KEY,
    release_id     TEXT NOT NULL REFERENCES release (id) ON DELETE CASCADE,
    low            REAL,
    median         REAL,
    high           REAL,
    lowest_listing REAL,
    num_for_sale   INTEGER,
    currency       TEXT,
    checked_at     TEXT NOT NULL
);
CREATE INDEX price_snapshot_release ON price_snapshot (release_id, checked_at);

CREATE TABLE streaming_link (
    release_id TEXT NOT NULL REFERENCES release (id) ON DELETE CASCADE,
    track_id   TEXT REFERENCES track (id) ON DELETE CASCADE,
    service    TEXT NOT NULL CHECK (service IN
                   ('spotify', 'apple_music', 'youtube', 'youtube_music', 'bandcamp')),
    url        TEXT NOT NULL,
    confirmed  INTEGER NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX streaming_link_key
    ON streaming_link (release_id, IFNULL(track_id, ''), service);

CREATE TABLE digital_album (
    id          TEXT PRIMARY KEY,
    release_id  TEXT REFERENCES release (id),
    folder_path TEXT NOT NULL,
    match_state TEXT NOT NULL CHECK (match_state IN ('suggested', 'confirmed', 'rejected'))
);

CREATE TABLE digital_file (
    id               TEXT PRIMARY KEY,
    digital_album_id TEXT NOT NULL REFERENCES digital_album (id) ON DELETE CASCADE,
    track_id         TEXT REFERENCES track (id),
    file_path        TEXT NOT NULL,
    format           TEXT,
    tags_written_at  TEXT
);

CREATE TABLE attribute (
    owner_id  TEXT NOT NULL,
    key       TEXT NOT NULL,
    value     TEXT,
    value_num REAL,
    PRIMARY KEY (owner_id, key)
);

CREATE TABLE change_log (
    id         TEXT PRIMARY KEY,
    table_name TEXT NOT NULL,
    row_id     TEXT NOT NULL,
    field      TEXT NOT NULL,
    new_value  TEXT,
    changed_at TEXT NOT NULL,
    device     TEXT NOT NULL
);
CREATE INDEX change_log_order ON change_log (changed_at, id);
