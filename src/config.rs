/// Configuration structure from environment variables.
use std::env;

/// Application configuration data.
#[derive(Clone)]
pub struct AppConfig {
    /// Host server IP (default localhost).
    pub host: String,
    /// Host server port (default 3000).
    pub port: u16,
    /// Rooms limit (default 50).
    pub max_rooms: usize,
    /// History limit (default 100).
    pub history_size: usize,
    /// Clients limit per room (deafult 50).
    pub max_room_clients: usize,
    ///
    pub heartbeat_s: u64,
    ///
    pub msg_rate_limit: u32,
    ///
    pub window_rate_limit_s: u64,
    ///
    pub max_msg_len: usize,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        dotenvy::dotenv().ok();

        AppConfig {
            host: register_env_var("HOST", "127.0.0.1".to_string()),
            port: register_env_var("PORT", 3000),
            max_rooms: register_env_var("ROOMS", 50),
            history_size: register_env_var("HISTORY", 100),
            max_room_clients: register_env_var("CLIENTS", 50),
            heartbeat_s: register_env_var("HEART", 30),
            msg_rate_limit: register_env_var("MSG_RATE", 10),
            window_rate_limit_s: register_env_var("WINDOW_RATE", 1),
            max_msg_len: register_env_var("MSG_LEN", 2048),
        }
    }
}

/// Process environmental variable for application config member.
fn register_env_var<T>(env_name: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    env::var(env_name)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
