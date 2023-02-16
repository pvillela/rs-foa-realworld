use chrono::{DateTime, Utc};

pub struct Following {
    pub follower_id: u64,
    pub followee_id: u64,
    pub followed_on: DateTime<Utc>,
}
