use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum AudioEvent {
    Playing { 
        track_id: i64, //tipo de dato entero de 64 bits
        position_ms: u64 //tipo de dato entero sin signo de 64 bits
    },
    
    Paused { 
        track_id: i64,
        position_ms: u64 
    },
    
    Stopped,
    
    TrackEnded { 
        track_id: i64 
    },
    
    TrackChanged { 
        previous_track_id: Option<i64>,
        current_track_id: i64,
        next_track_id: Option<i64>
    },
    
    VolumeChanged { 
        volume: f32
    },
    
    MuteToggled {
        is_muted: bool
    },
    
    ProgressUpdate {
        position_ms: u64,
        duration_ms: u64,
    },
    
    Error { 
        message: String,
        track_id: Option<i64>
    },
}

/// Tipo de callback para suscriptores de eventos
pub type EventCallback = Arc<dyn Fn(&AudioEvent) + Send + Sync>;

/// Bus de eventos del reproductor
/// 
/// Permite que diferentes partes de la aplicación se suscriban
/// y reaccionen a eventos de reproducción sin acoplamiento directo.
pub struct AudioEventBus {
    subscribers: Arc<Mutex<Vec<EventCallback>>>,
}

impl AudioEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn subscribe(&self, callback: EventCallback) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.push(callback);
    }
    
    pub fn emit(&self, event: AudioEvent) {
        let subs = self.subscribers.lock().unwrap();
        for callback in subs.iter() {
            callback(&event);
        }
    }
    
    //funcion para debug
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.lock().unwrap().len()
    }
}

impl Default for AudioEventBus {
    fn default() -> Self {
        Self::new()
    }
}