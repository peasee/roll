use std::sync::Arc;

use axum::{extract::State, response::Result, Extension};

use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    functions::recaptcha::Recaptcha,
    models::{AppConfiguration, AppState, NewPollBody, Poll},
};

use axum::{http::StatusCode, Json};

/// POST /poll
/// Create a new poll
///
/// # Errors
///
/// - missing recaptcha token, if enabled
/// - invalid poll
pub async fn create_poll(
    State(polls): State<Arc<RwLock<AppState>>>,
    Extension(config): Extension<Arc<AppConfiguration>>,
    Json(body): Json<serde_json::Value>, // it sucks that we can't just do: Json<NewPollBody>,
                                         // because for API servers, we can't control the error response in axum when the deserialize fails
                                         // see more: https://github.com/tokio-rs/axum/issues/1116
) -> Result<(StatusCode, Json<Value>)> {
    let body = NewPollBody::try_from(body)?; // instead, we have a TryFrom implementation which throws an APIError
                                             // here, we can control our response when the deserialization fails

    if body.recaptcha_token.is_some() {
        body.verify(Arc::clone(&config)).await?;
    }

    let mut guard = polls.write().await;
    let poll = Poll::from(body);
    guard.polls.insert(poll.id.clone(), poll.clone());

    Ok((StatusCode::CREATED, Json(json!({"id": poll.id}))))
}
