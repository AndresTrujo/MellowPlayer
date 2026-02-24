//! Entidad `Playlist` - lista de reproducción lógica.

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: i32,
    pub nombre: String,
    pub imagen: String,
}
