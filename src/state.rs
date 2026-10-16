/// State shared accross all handlers
use std::{collections::HashSet, sync::Mutex};

use tokio::sync::broadcast;

use crate::config::AppConfig;

/*
// includes
// standard
use std::sync::{
        atomic::AtomicUsize,
        Arc
};
use std::{
    collections::{HashMap,HashSet},
    time::Instant
};


use crate::peer::PeerHndl;
// user
use crate::{
    config::AppConfig,
    error::Result,
    room::Room
};
*/

/// Room state representation.
// Our shared state
pub struct AppState {
    // @todo currently app == single room, later user pool would be mapped into different rooms
    // We require unique usernames. This tracks which usernames have been taken.
    pub user_set: Mutex<HashSet<String>>,

    // @todo currently app == single room, later each room will have its broadcast
    /// Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new(cfg: AppConfig, tx: broadcast::Sender<String>) -> AppState {
        // @todo: for now app == single room, tweak later
        let user_set = Mutex::new(HashSet::with_capacity(cfg.max_room_clients));

        AppState { user_set, tx }
    }

    pub fn register_username(&self, name: &str) -> Option<String> {
        let mut user_set = self.user_set.lock().unwrap();
        let mut username = None;

        if !user_set.contains(name) {
            user_set.insert(name.to_owned());
            username = Some(name.to_owned());
        }

        username
    }

    pub fn remove_username(&self, username: &str) {
        self.user_set.lock().unwrap().remove(username);
    }
}

/*
/// Application State representation.
pub struct AppState {
    /// Available rooms to join.
    pub rooms:          Arc<HashMap<String, Room>>,
    /// App users.
    pub users:          Arc<HashSet<Arc<PeerHndl>>>,
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
            rooms:          Arc::new(HashMap::with_capacity(cfg.max_rooms)),
            users:          Arc::new(HashSet::with_capacity(cfg.max_room_clients)),
            total_clients:  Arc::new(AtomicUsize::new(0)),
            started_at:     Instant::now(),
            config:         Arc::new(cfg)
        }
    }

    /// Add new room
    pub async fn add_room(&mut self, name: &str) -> Result<()> {
        if !self.rooms.contains_key(name) {
            let mut rooms = Arc::get_mut(&mut self.rooms).ok_or("Failed to get mutable reference to rooms")?;
            rooms.insert(name.to_string(), Room::new(&self.config));
        }
        Ok(())
    }

    /*
    /// Timed (every 10 seconds) clean up action
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
*/
