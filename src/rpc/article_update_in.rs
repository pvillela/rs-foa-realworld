use crate::common::AppError;

pub struct ArticleUpdateIn {
    pub article: ArticleUpdateIn0,
}

pub struct ArticleUpdateIn0 {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

impl ArticleUpdateIn {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.article.slug == "" {
            return Err(AppError::ValidationFailed {
                msg: "article slug missing for Update operation".to_owned(),
            });
        }
        Ok(())
    }
}
