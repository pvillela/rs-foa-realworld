use crate::common::AppError;
use crate::model::Comment;

pub struct CommentAddIn {
    pub slug: String,
    pub comment: CommentAddIn0,
}

pub struct CommentAddIn0 {
    pub body: String, // mandatory
}

impl CommentAddIn {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.slug == "" || self.comment.body == "" {
            return Err(AppError::ValidationFailed {
                msg: "slug or body is missing".to_owned(),
            });
        }
        Ok(())
    }

    pub fn to_comment(self, article_id: u64, comment_author_id: u64) -> Comment {
        Comment::create(article_id, comment_author_id, self.comment.body)
    }
}
