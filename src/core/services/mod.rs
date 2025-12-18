//! Servicios de dominio que combinan repositorios para implementar reglas.
//!
//! P. ej. `LibraryService`, `PlaybackService`, `MetadataService`.

pub mod library_service;
pub mod playback_service;
pub mod metadata_service;

pub use library_service::LibraryService;
pub use playback_service::PlaybackService;
pub use metadata_service::MetadataService;
