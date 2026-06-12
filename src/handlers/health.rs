/// GET /health

// includes
// standard
use std::sync::Arc;
use axum::{
    extract::State,
    response::IntoResponse
};

// user
use crate::state::AppState;


/// GET some stats about server workflow.
pub async fn health_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {

}
