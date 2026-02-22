//! Entidad `Artist` - representación lógica de un artista.

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: u32;
    pub nombre: String;
    pub descripcion: String;
    pub imagen: String;
}
