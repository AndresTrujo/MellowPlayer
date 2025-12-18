//! Implementaciones concretas de repositorios usando `storage` (SQLite, etc.).

pub mod artist_repo_sqlite;
pub mod album_repo_sqlite;
pub mod track_repo_sqlite;
pub mod playlist_repo_sqlite;
pub mod audio_file_repo_sqlite;

pub use artist_repo_sqlite::ArtistRepoSqlite;
pub use album_repo_sqlite::AlbumRepoSqlite;
pub use track_repo_sqlite::TrackRepoSqlite;
pub use playlist_repo_sqlite::PlaylistRepoSqlite;
pub use audio_file_repo_sqlite::AudioFileRepoSqlite;
