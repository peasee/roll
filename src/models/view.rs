use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Poll;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollView {
    pub title: String,
    #[serde(rename = "optionCount")]
    pub option_count: u32,
    #[serde(flatten)]
    pub options: HashMap<String, String>,
}

impl From<&Poll> for PollView {
    fn from(poll: &Poll) -> Self {
        let mut view = PollView {
            title: poll.title.clone(),
            #[allow(clippy::cast_possible_truncation)]
            option_count: poll.options.len() as u32,
            options: HashMap::new(),
        };

        for (i, option) in poll.options.iter().enumerate() {
            view.options.insert(
                format!("{:#?}:count", i + 1),
                option.votes.len().to_string(),
            );
            view.options
                .insert(format!("{:#?}:prompt", i + 1), option.title.clone());
        }

        view
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollPartialView {
    #[serde(flatten)]
    pub options: HashMap<String, String>,
}

impl From<&Poll> for PollPartialView {
    fn from(poll: &Poll) -> Self {
        let mut view = PollPartialView {
            options: HashMap::new(),
        };

        for (i, option) in poll.options.iter().enumerate() {
            view.options
                .insert(format!("{:#?}", i + 1), option.votes.len().to_string());
        }

        view
    }
}
