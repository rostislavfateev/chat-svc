//! Example chat application.
//!
//! Run with
//!
//! ```not_rust
//! cargo run
//! ```

use chatsvc::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    chatsvc::init_tracing();

    let app_cfg = AppConfig::new();
    let listener = TcpListener::bind((app_cfg.host.as_str(), app_cfg.port)).await?;

    let result = chatsvc::run(listener, app_cfg).await;
    match &result {
        Ok(_) => tracing::debug!("Server exited gracefully"),
        Err(err) => tracing::error!("Server exited with error: {}", err),
    }
    result
}
