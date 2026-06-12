/// Configuration structure from environment variables.

use std::env;

/// Application configuration data.
pub struct AppConfig {
    /// Host server IP (default localhost).
    pub host:                   String,
    /// Host server port (default 3000).
    pub port:                   u16,
    /// Rooms limit (default 50).
    pub max_rooms:              usize,
    /// History limit (default 100).
    pub history_size:           usize,
    /// Clients limit (deafult 100).
    pub max_clients:            usize,
    /// 
    pub heartbeat_s:            u64,
    pub msg_rate_limit:         u32,
    pub window_rate_limit_s:    u64,
    pub max_msg_len:            usize,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        dotenvy::dotenv().ok();

        AppConfig {
            host:                   env::var("HOST")
                                        .expect("Hostname")
                                        .parse()
                                        .unwrap_or("127.0.0.1".to_string()),
            port:                   env::var("PORT")
                                        .expect("Port")
                                        .parse()
                                        .unwrap_or(300),
            max_rooms:              env::var("ROOMS")
                                        .expect("Max room count")
                                        .parse()
                                        .unwrap_or(50),
            history_size:           env::var("HISTORY")
                                        .expect("History size")
                                        .parse()
                                        .unwrap_or(100),
            max_clients:            env::var("CLIENTS")
                                        .expect("Max simultaneous clients")
                                        .parse()
                                        .unwrap_or(100),
            heartbeat_s:            env::var("HEART")
                                        .expect("Heartbeat interval in seconds")
                                        .parse()
                                        .unwrap_or(30),
            msg_rate_limit:         env::var("MSG_RATE")
                                        .expect("Message rate limit")
                                        .parse()
                                        .unwrap_or(10),
            window_rate_limit_s:    env::var("WINDOW_RATE")
                                        .expect("Window rate limit in seconds")
                                        .parse()
                                        .unwrap_or(1),
            max_msg_len:            env::var("MSG_LEN")
                                        .expect("Max message length")
                                        .parse()
                                        .unwrap_or(2048),
        }
    }
}
