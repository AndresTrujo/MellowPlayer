# Mellomaniac (esqueleto)

Proyecto: reproductor musical de escritorio — esqueleto arquitectónico.

Propósito
- Proveer una base de código con arquitectura limpia (UI / Application / Core / Storage / Audio / Scanner).
- Implementación inicial de la base de datos SQLite a partir del script en `data/migrations/001_initial_schema.sql`.

Estructura relevante
- `src/` : código Rust (ver módulos `app`, `core`, `storage`, `audio`, `scanner`, `ui`, `utils`).
- `data/migrations/001_initial_schema.sql` : esquema inicial de la base de datos (tabla `artist`, `album`, `track`, `audio_file`, etc.).

Inicializar la base de datos

Al ejecutar el binario, el proceso intentará crear/abrir `mellomaniac.db` en el directorio actual
y aplicará la migración inicial. Esto se realiza a través de `storage::sqlite::SqliteConnection::open_and_migrate`.

Ejemplo:

```bash
cargo run --release
# o en modo dev
cargo run
```

Después de la ejecución deberías ver un archivo `mellomaniac.db` con las tablas definidas en
`data/migrations/001_initial_schema.sql`.

Notas
- Por ahora la aplicación sólo aplica la migración inicial y no expone una CLI para gestión.
- No se ha implementado lógica de negocio, UI real, ni reproducción de audio.
- El código sigue el principio de desacoplamiento: `core` define contratos; `storage` contiene adaptadores.

¿Siguientes pasos sugeridos?
- Implementar repositorios concretos que usen la conexión SQLite.
- Añadir gestión de migraciones más robusta (versionado incremental).
- Implementar eventos y cola entre `app`, `audio` y `storage`.
