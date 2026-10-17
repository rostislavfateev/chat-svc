//! src/lib.rs
//!

use axum::{Router, response::Html, routing::get};

use std::sync::Arc;
use tokio::{net::TcpListener, signal, sync::broadcast};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// User modules
pub mod config;
pub mod peer;
pub mod state;
pub mod ws;

use crate::config::AppConfig;
use crate::state::AppState;
use crate::ws::connection::websocket_handler;

/// Initialize the global tracing subscriber. Call once, from `main`.
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Build the application router.
pub fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health_check", get(health_check_handler))
        .route("/websocket", get(websocket_handler))
        .with_state(state)
}

/// Serve the application on an already bound listener until shutdown.
pub async fn run(listener: TcpListener, app_cfg: AppConfig) -> std::io::Result<()> {
    let (tx, _rx) = broadcast::channel(app_cfg.max_room_clients);
    let app_state = Arc::new(AppState::new(app_cfg, tx));

    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app(app_state))
        .with_graceful_shutdown(shutdown_signal())
        .await
}

// Standard Handlers
/// Serves the chat page.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../html/chat.html"))
}

/// Serves success if application is up.
async fn health_check_handler() -> &'static str {
    "OK"
}

/// Graceful shutdown handling
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
