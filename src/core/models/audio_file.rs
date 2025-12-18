//! `AudioFile` - representación del archivo físico en disco.
//!
//! Separado de `Track` para distinguir entre metadatos de pista y archivo.

#[derive(Debug, Clone)]
pub struct AudioFile {
    // TODO: path, size, hash, mime/type, associated track id (optional)
}
