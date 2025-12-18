//! Conexión SQLite y ejecución de migraciones iniciales.
//!
//! Esta implementación inicial abre una base de datos SQLite (archivo) y
//! ejecuta el script de migración encontrado en `data/migrations/001_initial_schema.sql`.
//!
//! Nota: es un adaptador mínimo para el esqueleto; no maneja pooling ni migraciones
//! incrementales todavía.

use std::fs;
use std::path::PathBuf;

use rusqlite::{Connection, Result as SqlResult};

pub struct SqliteConnection {
    pub inner: Connection,
}

impl SqliteConnection {
    /// Abre (o crea) la base de datos en `db_path` y aplica la migración inicial.
    pub fn open_and_migrate(db_path: &str) -> SqlResult<Self> {
        // Abrir/crear la base de datos
        let conn = Connection::open(db_path)?;

        // Cargar migración desde `data/migrations/001_initial_schema.sql`.
        // Usamos `CARGO_MANIFEST_DIR` para localizar el archivo relativo al proyecto.
        let manifest = env!("CARGO_MANIFEST_DIR");
        let mut mig_path = PathBuf::from(manifest);
        mig_path.push("data/migrations/001_initial_schema.sql");

        let sql = fs::read_to_string(&mig_path).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;

        // Ejecutar script (puede contener múltiples sentencias)
        conn.execute_batch(&sql)?;

        Ok(SqliteConnection { inner: conn })
    }
}
