use crate::model::{Article, User};
use anyhow::{anyhow, Error, Result};

pub struct ArticleCreateIn {
    pub article: ArticleCreateIn0,
}

pub struct ArticleCreateIn0 {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

impl ArticleCreateIn {
    pub fn validate(&self) -> Result<(), Error> {
        if self.article.title == "" || self.article.description == "" || self.article.body == "" {
            return Err(anyhow!("article has missing fields for Create operation"));
        }
        Ok(())
    }

    pub fn to_article(self, author: User) -> Article {
        let mut tag_list = Vec::new();
        if let Some(in_tag_list) = self.article.tag_list {
            tag_list = in_tag_list;
        }

        Article::create(
            author,
            self.article.title,
            self.article.description,
            self.article.body,
            tag_list,
        )
    }
}
