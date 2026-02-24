//! Entidad `Track` - representación lógica de una pista musical.
//!
//! Debe permanecer separada de `AudioFile` (archivo físico en disco).

#[derive(Debug, Clone)]
pub struct Track {
    pub id: u64,
    pub titulo: String,
    pub duracion_segundos: f32,
    pub numero_pista: i32,
    pub album_id: i64,
}
