//! Entidad `Album` - representación lógica de un álbum.
#[derive(Debug, Clone)]
pub struct Album {
    pub id: i64,
    pub nombre: String,
    pub fecha_lanzamiento: Option<NaiveDate>,
    pub imagen: Option<String>,
    pub artist_id: i64,
}
