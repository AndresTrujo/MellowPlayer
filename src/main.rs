// Entry point: inicializa los puntos de integración (UI, audio, storage).
// No debe contener lógica de negocio pesada.

mod storage;

use storage::sqlite::SqliteConnection;

fn main() {
    // TODO: Inicializar logger
    println!("mellomaniac: inicializando aplicación (esqueleto)");

    // Intentar inicializar la DB SQLite con la migración inicial.
    match SqliteConnection::open_and_migrate("mellomaniac.db") {
        Ok(_) => println!("Base de datos inicializada correctamente."),
        Err(e) => eprintln!("Error inicializando la base de datos: {}", e),
    }

    // Punto de entrada UI: inicializar GTK / libadwaita (no implementado aquí)
    // Punto de entrada Audio: iniciar motor (p. ej. GStreamer) (no implementado aquí)

    // La orquestación real se implementará en `app` y `core`.
}
