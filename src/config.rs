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
    /// Clients limit per room (deafult 50).
    pub max_room_clients:       usize,
    /// 
    pub heartbeat_s:            u64,
    ///
    pub msg_rate_limit:         u32,
    ///
    pub window_rate_limit_s:    u64,
    /// 
    pub max_msg_len:            usize,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        dotenvy::dotenv().ok();

        AppConfig {
            host:                env::var("HOST").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or("127.0.0.1".to_string()),
            port:                env::var("PORT").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(3000),
            max_rooms:           env::var("ROOMS").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(50),
            history_size:        env::var("HISTORY").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(100),
            max_room_clients:    env::var("CLIENTS").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(50),
            heartbeat_s:         env::var("HEART").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(30),
            msg_rate_limit:      env::var("MSG_RATE").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(10),
            window_rate_limit_s: env::var("WINDOW_RATE").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(1),
            max_msg_len:         env::var("MSG_LEN").ok().and_then(|s| s.parse().ok())
                                    .unwrap_or(2048),
        }
    }
}
