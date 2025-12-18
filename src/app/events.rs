//! Eventos emitidos por la capa `app`.
//!
//! Diseñados para desacoplar UI, audio y storage.

#[derive(Debug)]
pub enum AppEvent {
    // TODO: LibraryScanned, TrackStarted(track_id), PlaybackStopped, Error...
    LibraryScanned,
}
