//! Controlador de la aplicación: conecta acciones con casos de uso y servicios.
//!
//! Orquesta llamadas a `core::use_cases` y publica eventos.

use crate::app::events::AppEvent;

pub struct AppController {
    // TODO: referencias a servicios, repositorios y canales de eventos
}

impl AppController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_action(&self) -> Option<AppEvent> {
        // TODO: implementar en el futuro
        None
    }
}
