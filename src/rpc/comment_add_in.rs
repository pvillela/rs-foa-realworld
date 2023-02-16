use crate::model::Comment;
use anyhow::{anyhow, Result};

pub struct CommentAddIn {
    pub slug: String,
    pub comment: CommentAddIn0,
}

pub struct CommentAddIn0 {
    pub body: String, // mandatory
}

impl CommentAddIn {
    pub fn validate(&self) -> Result<()> {
        if self.slug == "" || self.comment.body == "" {
            return Err(anyhow!("slug or body is missing"));
        }
        Ok(())
    }

    pub fn to_comment(self, article_id: u64, comment_author_id: u64) -> Comment {
        Comment::create(article_id, comment_author_id, self.comment.body)
    }
}
