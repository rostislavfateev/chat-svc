///

// includes
// standard
use tokio::sync::broadcast::{self, Sender};
use std::collections::HashSet;

use std::{
    collections::{
        VecDeque
    },
    sync::{Arc, Mutex}
};

// user
use crate::{
    config::AppConfig, error::AppError, peer::PeerHndl, state::AppState, ws::protocol::ServerMsg
};


pub struct Room {
    /// Broadcast channel.
    pub tx:             broadcast::Sender<ServerMsg>,
    pub users:          HashSet<Arc<PeerHndl>>,
    /// Chat history.
    pub history:        VecDeque<ServerMsg>
}

impl Room {
    pub fn new(cfg: &AppConfig) -> Room {
        Room {
            tx:         broadcast::Sender::new(cfg.max_room_clients),
            users:      HashSet::with_capacity(cfg.max_room_clients),
            history:    VecDeque::with_capacity(cfg.history_size),
        }
    }

    pub async fn add_user(&mut self, user: Arc<PeerHndl>) -> Result<(), AppError> {
        if !self.users.contains(&user) {
            let mut users = Arc::get_mut(&mut self.users).ok_or("Failed to get mutable reference to users")?;
            users.insert(user.clone());
        }
        Ok(())
    }
}




/*
pub async fn get_or_create_room(state: &AppState, room_id: &str)
    -> Result<broadcast::Sender<ServerMsg>, AppError> {
    // @todo implement
    Ok(Sender::new(0))
}


pub async fn leave_room(state: &AppState, room_id: &str) {
    // @todo implement
}
 */