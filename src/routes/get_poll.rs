use std::sync::Arc;

use axum::{extract::State, response::Result};

use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::models::{APIError, AppState, PollView};

use axum::{extract::Path, http::StatusCode, Json};

/// GET /poll/:id
/// Returns a poll with the given id
///
/// # Errors
///
/// - missing poll
pub async fn get_poll(
    Path(id): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<(StatusCode, Json<Value>)> {
    let guard = state.read().await;

    match guard.polls.get(&id) {
        Some(poll) => {
            let view = PollView::from(poll);

            Ok((StatusCode::OK, Json(json!(view))))
        }
        None => Err(APIError::PollNotFound.into()),
    }
}
