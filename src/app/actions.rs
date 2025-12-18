//! Acciones de alto nivel que la UI puede despachar.
//!
//! Contiene definiciones de comandos / intentos que se traducirán a casos de uso.

#[derive(Debug)]
pub enum AppAction {
    // TODO: ScanLibrary, PlayTrack(Id), Pause, Seek, etc.
    ScanLibrary,
}
