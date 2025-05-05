use std::{future::Future, sync::Arc};

use crate::models::{APIError, AppConfiguration, NewPollBody, PollVoteBody};
use axum::response::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RecaptchaResponse {
    success: bool,
    hostname: String,
    action: String,
    score: f32,
}

async fn verify_recaptcha(
    config: &Arc<AppConfiguration>,
    recaptcha_token: &str,
    originating_route: &str,
) -> anyhow::Result<bool> {
    let secret_key = config.recaptcha_secret_key.clone().unwrap_or_default();
    let client = reqwest::Client::new();
    let res = client.post(format!("https://www.google.com/recaptcha/api/siteverify?secret={secret_key}&response={recaptcha_token}"))
        .send()
        .await?.json::<RecaptchaResponse>().await?;

    let valid = res.success
        && res.hostname == config.host
        && res.action == originating_route
        && res.score >= 0.5;

    Ok(valid)
}

pub trait Recaptcha {
    /// # Errors
    ///
    /// - missing recaptcha secret or site key from configuration
    fn validate(&self, config: &Arc<AppConfiguration>) -> Result<(), APIError> {
        if config.recaptcha_secret_key.is_none() || config.recaptcha_site_key.is_none() {
            return Err(APIError::ConfigurationError);
        }

        Ok(())
    }

    fn verify(
        &self,
        config: Arc<AppConfiguration>,
    ) -> impl Future<Output = Result<(), APIError>> + Send;
}

impl Recaptcha for PollVoteBody {
    async fn verify(&self, config: Arc<AppConfiguration>) -> Result<(), APIError> {
        // verify the recaptcha token
        // this is a stub implementation
        self.validate(&config)?;

        let recaptcha_response = verify_recaptcha(
            &config,
            self.recaptcha_token.as_ref().unwrap_or(&String::new()),
            "vote",
        )
        .await;

        match recaptcha_response {
            Ok(true) => Ok(()),
            Ok(false) => Err(APIError::InvalidToken),
            Err(_) => Err(APIError::ConfigurationError),
        }
    }
}

impl Recaptcha for NewPollBody {
    async fn verify(&self, config: Arc<AppConfiguration>) -> Result<(), APIError> {
        // verify the recaptcha token
        // this is a stub implementation
        self.validate(&config)?;

        let recaptcha_response = verify_recaptcha(
            &config,
            self.recaptcha_token.as_ref().unwrap_or(&String::new()),
            "create",
        )
        .await;

        match recaptcha_response {
            Ok(true) => Ok(()),
            Ok(false) => Err(APIError::InvalidToken),
            Err(_) => Err(APIError::ConfigurationError),
        }
    }
}
