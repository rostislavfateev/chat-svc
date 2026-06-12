///

// includes
// standard
use std::collections::{
    HashMap,
    VecDeque
};
use std::sync::{
    atomic::AtomicUsize,
    Arc,
    RwLock
};
use std::time::Instant;
use tokio::sync::broadcast;

// user
use crate::{
    config::AppConfig,
    message::ServerMsg
};


/// Room state representation.
pub struct RoomState {
    /// Broadcast channel.
    pub tx:             broadcast::Sender<ServerMsg>,
    /// Chat history.
    pub history:        VecDeque<ServerMsg>,
    /// Number of clients connected.
    pub client_cnt:     usize,
}

impl RoomState {
    pub fn new(cfg: &AppConfig) -> RoomState {
        RoomState {
            tx:         broadcast::Sender::new(cfg.max_clients),
            history:    VecDeque::new(),//.reserve(cfg.history_size),
            client_cnt: cfg.max_clients,
        }

    }

}


/// Application State representation.
pub struct AppState {
    /// Available rooms to join.
    pub rooms:          Arc<RwLock<HashMap<String, RoomState>>>,
    /// Number of clients connected.
    pub total_clients:  Arc<AtomicUsize>,
    /// App start time.
    pub started_at:     Instant,
    /// Application configuration.
    pub config:         Arc<AppConfig>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            rooms:          Arc::new(RwLock::new(HashMap::new())),
            total_clients:  Arc::new(AtomicUsize::new(0)),
            started_at:     Instant::now(),
            config:         Arc::new(AppConfig::new())
        }
    }

    pub fn add_room() {
        // @todo implement
    }

}
