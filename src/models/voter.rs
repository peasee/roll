use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct Voter {
    pub ip: String,
    pub poll_id: String,
    pub option_id: u64,
}

impl Voter {
    #[must_use]
    pub fn new(ip: String, poll_id: String, option_id: u64) -> Self {
        Voter {
            ip,
            poll_id,
            option_id,
        }
    }
}
