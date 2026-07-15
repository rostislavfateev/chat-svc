/// Entry point
/// - config
/// - tracing init
/// - router
/// - shutdown

use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};
use tokio::{
    signal,
    sync::broadcast
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// user includes
pub mod state;
pub mod room;
pub mod message;
pub mod config;
pub mod error;
pub mod handlers;

use crate::{
    handlers::{
        health::health_handler,
        websocket::websocket_handler
    },
    state::AppState
};




/* 
// Our shared state
struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    user_set: Mutex<HashSet<String>>,
    // Channel used to send messages to all connected clients.
    tx: broadcast::Sender<String>,
}
*/



#[tokio::main]
async fn main() {
    tracing_subscriber_init();

    // Set up application state for use with with_state().
    //let user_set = Mutex::new(HashSet::new());
    //let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState::new());
    let addr = format!("{}:{}", app_state.config.host, app_state.config.port);

    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(websocket_handler))
        .route("/health", get(health_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
}


// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../html/chat.html"))
}


/// Debug info 
async fn tracing_subscriber_init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
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
