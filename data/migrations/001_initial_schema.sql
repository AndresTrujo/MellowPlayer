PRAGMA foreign_keys = ON;

-- ─────────────────────────────
-- ARTIST
-- ─────────────────────────────
CREATE TABLE artist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    descripcion TEXT,
    imagen TEXT
);

-- ─────────────────────────────
-- ALBUM
-- ─────────────────────────────
CREATE TABLE album (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    fecha_lanzamiento DATE,
    imagen TEXT,
    artist_id INTEGER NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artist(id)
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

-- ─────────────────────────────
-- TRACK
-- ─────────────────────────────
CREATE TABLE track (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    titulo TEXT NOT NULL,
    duracion_segundos INTEGER NOT NULL,
    numero_pista INTEGER,
    album_id INTEGER,
    FOREIGN KEY (album_id) REFERENCES album(id)
        ON UPDATE CASCADE
        ON DELETE SET NULL
);

-- ─────────────────────────────
-- PLAYLIST
-- ─────────────────────────────
CREATE TABLE playlist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    imagen TEXT
);

-- ─────────────────────────────
-- PLAYLIST_TRACK (N-N con orden)
-- ─────────────────────────────
CREATE TABLE playlist_track (
    playlist_id INTEGER NOT NULL,
    track_id INTEGER NOT NULL,
    posicion INTEGER NOT NULL,

    PRIMARY KEY (playlist_id, track_id),
    UNIQUE (playlist_id, posicion),

    FOREIGN KEY (playlist_id) REFERENCES playlist(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    FOREIGN KEY (track_id) REFERENCES track(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

-- ─────────────────────────────
-- TRACK_ARTIST (N-N)
-- ─────────────────────────────
CREATE TABLE track_artist (
    track_id INTEGER NOT NULL,
    artist_id INTEGER NOT NULL,
    rol TEXT,

    PRIMARY KEY (track_id, artist_id),

    FOREIGN KEY (track_id) REFERENCES track(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    FOREIGN KEY (artist_id) REFERENCES artist(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

-- ─────────────────────────────
-- GENRE
-- ─────────────────────────────
CREATE TABLE genre (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL UNIQUE
);

-- ─────────────────────────────
-- ALBUM_GENRE (N-N)
-- ─────────────────────────────
CREATE TABLE album_genre (
    album_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,

    PRIMARY KEY (album_id, genre_id),

    FOREIGN KEY (album_id) REFERENCES album(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    FOREIGN KEY (genre_id) REFERENCES genre(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

-- ─────────────────────────────
-- TRACK_GENRE (N-N)
-- ─────────────────────────────
CREATE TABLE track_genre (
    track_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,

    PRIMARY KEY (track_id, genre_id),

    FOREIGN KEY (track_id) REFERENCES track(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    FOREIGN KEY (genre_id) REFERENCES genre(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

-- ─────────────────────────────
-- LIBRARY_ROOT
-- ─────────────────────────────
CREATE TABLE library_root (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    last_scan DATETIME
);

-- ─────────────────────────────
-- AUDIO_FILE (archivo físico)
-- ─────────────────────────────
CREATE TABLE audio_file (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    track_id INTEGER,
    library_root_id INTEGER NOT NULL,

    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    extension TEXT NOT NULL,

    file_size INTEGER NOT NULL,
    modified_at DATETIME NOT NULL,
    duration_segundos INTEGER,

    hash TEXT,
    bitrate INTEGER,
    sample_rate INTEGER,
    channels INTEGER,

    FOREIGN KEY (track_id) REFERENCES track(id)
        ON UPDATE CASCADE
        ON DELETE SET NULL,

    FOREIGN KEY (library_root_id) REFERENCES library_root(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

-- ─────────────────────────────
-- RAW_METADATA (tags crudos)
-- ─────────────────────────────
CREATE TABLE raw_metadata (
    audio_file_id INTEGER PRIMARY KEY,

    title TEXT,
    album TEXT,
    artist TEXT,
    album_artist TEXT,
    track_number INTEGER,
    disc_number INTEGER,
    year INTEGER,
    genre TEXT,

    FOREIGN KEY (audio_file_id) REFERENCES audio_file(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
