use std::collections::HashSet;

use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use super::Voter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PollOption {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub votes: HashSet<Voter>,
}

impl PollOption {
    pub fn new(title: String, description: Option<String>) -> Self {
        PollOption {
            id: nanoid!(),
            title,
            description,
            votes: HashSet::new(),
        }
    }

    pub fn vote(&mut self, voter: &Voter) -> bool {
        self.votes.insert(voter.clone())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Poll {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<PollOption>,
}

impl Poll {
    pub fn new(title: String, description: Option<String>) -> Self {
        Poll {
            id: nanoid!(),
            title,
            description,
            options: Vec::new(),
        }
    }

    pub fn add_option(&mut self, option: PollOption) {
        self.options.push(option);
    }

    pub fn remove_option(&mut self, option: &PollOption) {
        self.options.retain(|o| o.id != option.id);
    }
}
