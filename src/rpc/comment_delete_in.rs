use crate::common::AppError;

pub struct CommentDeleteIn {
    pub slug: String,
    pub id: u64,
}

impl CommentDeleteIn {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.slug == "" || self.id == 0 {
            return Err(AppError::ValidationFailed {
                msg: "article slug or comment id is missing".to_owned(),
            });
        }
        Ok(())
    }
}
