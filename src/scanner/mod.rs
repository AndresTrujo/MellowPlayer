//! `scanner` - escaneo del sistema de archivos para descubrir audio.
//!
//! Separa responsabilidad: hashing, lectura de tags, scheduler, filesystem walker.

pub mod filesystem;
pub mod hash;
pub mod tag_reader;
pub mod scheduler;

pub use filesystem::FilesystemScanner;
