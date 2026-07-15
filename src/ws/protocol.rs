/// Message implementation.

// includes
// standard
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

// user


#[derive(Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum ClientMsg {
    Join {
        username:   String,
        room:       String
    },
    Message {
        content:    String
    },
    SwitchRoom {
        room:       String
    },
    Pong,
    Leave,
}


#[derive(Serialize, Clone)]
#[serde(tag="type", rename_all="snake_case")]
pub enum ServerMsg {
    Joined {
        client_id:  Uuid,
        room:       String,
        history:    Vec<ServerMsg>,
    },
    Message {
        id:         Uuid,
        sender:     String,
        room:       String,
        content:    String,
        timestamp:  DateTime<Utc>,
    },
    System {
        room:       String,
        content:    String,
        timestamp:  DateTime<Utc>,
    },
    Ping {
        timestamp:  DateTime<Utc>,
    },
    Error {
        // @todo make it enum? 
        // RATE_LIMITED | ROOM_LIMIT_REACHED | INVALID_MESSAGE | ...
        code:       String,
        message:    String,
    },
}
