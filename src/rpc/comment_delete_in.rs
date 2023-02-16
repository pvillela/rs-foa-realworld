use anyhow::{anyhow, Result};

pub struct CommentDeleteIn {
    pub slug: String,
    pub id: u64,
}

impl CommentDeleteIn {
    pub fn validate(&self) -> Result<()> {
        if self.slug == "" || self.id == 0 {
            return Err(anyhow!("article slug or comment id is missing"));
        }
        Ok(())
    }
}
