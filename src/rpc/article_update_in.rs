use anyhow::{anyhow, Result};

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
    pub fn validate(&self) -> Result<()> {
        if self.article.slug == "" {
            return Err(anyhow!("article slug missing for Update operation"));
        }
        Ok(())
    }
}
