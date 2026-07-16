/// State shared accross all handlers

// includes
// standard
use std::sync::{
        atomic::AtomicUsize,
        Arc
};
use std::time::Instant;
use dashmap::DashMap;

use crate::peer::PeerHndl;
// user
use crate::{
    config::AppConfig,
    error::Result,
    room::Room
};


/// Room state representation.



/// Application State representation.
pub struct AppState {
    /// Available rooms to join.
    pub rooms:          Arc<DashMap<String, Room>>,
    /// App users.
    pub users:          Arc<DashMap<String, Arc<PeerHndl>>>,
    /// Number of clients connected.
    pub total_clients:  Arc<AtomicUsize>,
    /// App start time.
    pub started_at:     Instant,
    /// Application configuration.
    pub config:         Arc<AppConfig>,
}

impl AppState {
    pub fn new() -> AppState {
        let cfg = AppConfig::new();
        AppState {
            rooms:          Arc::new(DashMap::with_capacity(cfg.max_rooms)),
            users:          Arc::new(DashMap::with_capacity(cfg.max_room_clients)),
            total_clients:  Arc::new(AtomicUsize::new(0)),
            started_at:     Instant::now(),
            config:         Arc::new(cfg)
        }
    }

    /// Add new room
    pub async fn add_room(&mut self, name: &str) -> Result<()> {
        if !self.rooms.contains_key(name) {
            self.rooms.insert(name.to_string(), Room::new(&self.config));
        }
        Ok(())
    }

    /*
    pub async fn cleanup(&mut self) {
        let rooms = Arc::clone(&self.rooms);

        tokio::spawn( async move {

            // @todo better way than saving keys
            for pair in rooms.iter_mut() {
                if pair.value().client_cnt == 0 {
                    rooms.remove(pair.key());
                }
            }

        });
    }
     */
}
