//! `audio` - motor de audio y adaptadores (p. ej. GStreamer).
//!
//! Este módulo expone interfaces mínimas y eventos para desacoplar reproducción.

pub mod player;
pub mod pipeline;
pub mod events;

pub use player::AudioPlayer;
pub use events::AudioEvent;
