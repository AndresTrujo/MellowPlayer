//! `app` - capa de orquestación y puntos de entrada de la aplicación.
//!
//! Responsable de inicializar subsistemas (UI, audio, storage) y de
//! mantener el estado de alto nivel. No contiene lógica de negocio.

#![allow(dead_code, unused_imports)]

pub mod state;
pub mod actions;
pub mod controller;
pub mod events;
