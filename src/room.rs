///

// includes
// standard
use tokio::sync::broadcast;

// user
use crate::{
    error::AppError,
    message::ServerMsg,
    state::AppState
};

pub async fn get_or_create_room(state: &AppState, room_id: &str)
    -> Result<broadcast::Sender<ServerMsg>, AppError> {
    // @todo implement
}


pub async fn leave_room(state: &AppState, room_id: &str) {
    // @todo implement
}
