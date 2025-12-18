//! Modelos de dominio: `Track`, `Artist`, `Album`, `Playlist`, `AudioFile`, `Genre`.
//!
//! Aquí sólo definimos estructuras y responsabilidades, sin lógica.

pub mod artist;
pub mod album;
pub mod track;
pub mod playlist;
pub mod audio_file;
pub mod genre;

pub use artist::Artist;
pub use album::Album;
pub use track::Track;
pub use playlist::Playlist;
pub use audio_file::AudioFile;
pub use genre::Genre;
