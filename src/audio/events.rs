//! Eventos relacionados con el motor de audio.

#[derive(Debug)]
pub enum AudioEvent {
    // TODO: Playing(track_id), Paused, Stopped, Error
    Stopped,
}
