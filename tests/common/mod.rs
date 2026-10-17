//! Shared helpers for integration tests.

use chatsvc::config::AppConfig;
use tokio::net::TcpListener;

/// Spawn the app on a random free port and return its base URL.
pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    tokio::spawn(chatsvc::run(listener, AppConfig::new()));

    format!("http://{addr}")
}
