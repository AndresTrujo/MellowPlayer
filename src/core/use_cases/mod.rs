//! Casos de uso de la aplicación (orquestan servicios y repositorios).

pub mod scan_library;
pub mod play_track;
pub mod play_playlist;
pub mod search;
pub mod resolve_metadata;

pub use scan_library::ScanLibrary;
pub use play_track::PlayTrack;
pub use play_playlist::PlayPlaylist;
pub use search::Search;
pub use resolve_metadata::ResolveMetadata;
