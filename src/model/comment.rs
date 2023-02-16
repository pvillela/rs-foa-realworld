use chrono::{DateTime, Utc};

pub struct Comment {
    pub id: u64,
    pub article_id: u64,
    pub author_id: u64,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn create(article_id: u64, author_id: u64, body: String) -> Comment {
        return Comment {
            id: 0,
            article_id,
            author_id,
            body,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
    }
}
