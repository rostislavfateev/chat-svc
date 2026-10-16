//! Example chat application.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-chat
//! ```

use axum::{Router, response::Html, routing::get};

use std::sync::Arc;
use tokio::{signal, sync::broadcast};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// User modules
pub mod config;
pub mod peer;
pub mod state;
pub mod ws;

use crate::config::AppConfig;
use crate::state::AppState;
use crate::ws::connection::websocket_handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Application configuration set up
    let app_cfg = AppConfig::new();
    // @todo for now app == single room, tweak later
    let (tx, _rx) = broadcast::channel(app_cfg.max_room_clients);
    let app_state = Arc::new(AppState::new(app_cfg.clone(), tx));

    // handle connections
    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    // actual server run
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", app_cfg.host, app_cfg.port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;

    match result {
        Ok(_) => tracing::debug!("Server exited gracefully"),
        Err(err) => tracing::error!("Server exited with error: {}", err),
    }
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../html/chat.html"))
}

/// Graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
