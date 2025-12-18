//! Contratos de repositorio (interfaces) para acceso a datos del dominio.
//!
//! Implementaciones concretas (SQLite, etc.) vivirán en `storage::repositories`.

pub mod artist_repo;
pub mod album_repo;
pub mod track_repo;
pub mod playlist_repo;
pub mod audio_file_repo;

pub use artist_repo::ArtistRepository;
pub use album_repo::AlbumRepository;
pub use track_repo::TrackRepository;
pub use playlist_repo::PlaylistRepository;
pub use audio_file_repo::AudioFileRepository;
