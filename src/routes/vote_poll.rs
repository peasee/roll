use std::sync::Arc;

use axum::{extract::State, http::HeaderMap, response::Result, Extension};

use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    functions::recaptcha::Recaptcha,
    models::{APIError, AppConfiguration, AppState, PollVoteBody, Voter},
};

use axum::{extract::Path, http::StatusCode, Json};

use axum::extract::ConnectInfo;
use std::net::SocketAddr;

/// POST /poll/:id/vote
/// Vote on a poll
///
/// # Errors
///
/// - missing poll/option
/// - missing recaptcha token, if enabled
/// - invalid poll/option
pub async fn vote_poll(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(polls): State<Arc<RwLock<AppState>>>,
    Extension(config): Extension<Arc<AppConfiguration>>,
    Json(body): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<Value>)> {
    let body = PollVoteBody::try_from(body)?;
    if body.recaptcha_token.is_some() {
        body.verify(Arc::clone(&config)).await?;
    }

    let mut guard = polls.write().await;

    match guard.polls.get_mut(&id) {
        Some(poll) => {
            // the option is the array index of the target option + 1
            #[allow(clippy::cast_possible_truncation)]
            let option = poll
                .options
                .get_mut((body.option - 1) as usize)
                .ok_or(APIError::OptionNotFound)?;

            let ip = if let Some(ip) = headers.get("X-Forwarded-For") {
                ip.to_str().unwrap_or(&addr.to_string()).to_string()
            } else {
                addr.to_string()
            };

            tracing::info!(ip, "Voting on poll");
            let voter = Voter::new(ip, id.clone(), body.option);
            let has_voted = option.vote(&voter);

            Ok((StatusCode::OK, Json(json!({"voted": has_voted}))))
        }
        None => Err(APIError::PollNotFound.into()),
    }
}
