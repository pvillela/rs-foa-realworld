use crate::arch::util;
use crate::model::{Profile, User};
use arcstr::ArcStr;
use chrono::{DateTime, Utc};

pub struct Article {
    pub id: u64,
    pub author_id: u64,
    pub title: String,
    pub slug: String,
    pub description: ArcStr,
    pub body: ArcStr,
    pub favorites_count: i64,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ArticlePlus {
    pub id: u64,
    pub slug: String,
    pub author: Profile,
    pub title: String,
    pub description: ArcStr,
    pub body: ArcStr,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i64,
}

pub struct ArticlePatch {
    pub title: Option<String>,
    pub description: Option<ArcStr>,
    pub body: Option<ArcStr>,
    pub tag_list: Option<Vec<String>>,
}

impl Article {
    pub fn create(
        author: User,
        title: &str,
        description: &str,
        body: &str,
        tag_list: &Vec<String>,
    ) -> Article {
        Article {
            id: 0,
            author_id: author.id,
            title: title.to_owned(),
            slug: util::slug(title),
            description: ArcStr::from(description),
            body: ArcStr::from(body),
            favorites_count: 0,
            tag_list: tag_list.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update(&mut self, src: ArticlePatch) {
        if let Some(title) = src.title {
            self.title = title;
        }
        if let Some(description) = src.description {
            self.description = description;
        }
        if let Some(body) = src.body {
            self.body = body;
        }
        if let Some(tag_list) = src.tag_list {
            self.tag_list = tag_list;
        }
    }

    pub fn with_adjusted_favorite_count(&mut self, delta: i64) {
        self.favorites_count += delta;
    }

    pub fn to_article_plus(&self, favorited: bool, author: Profile) -> ArticlePlus {
        return ArticlePlus {
            id: self.id,
            slug: self.slug.clone(),
            author: Profile {
                user_id: author.user_id,
                username: author.username,
                bio: author.bio,
                image: author.image,
                following: author.following,
            },
            title: self.title.clone(),
            description: self.description.clone(),
            body: self.body.clone(),
            tag_list: self.tag_list.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited: favorited,
            favorites_count: self.favorites_count,
        };
    }
}

impl ArticlePlus {
    pub fn to_article(&self) -> Article {
        Article {
            id: self.id,
            author_id: self.author.user_id,
            title: self.title.to_owned(),
            slug: self.slug.to_owned(),
            description: self.description.to_owned(),
            body: self.body.clone(),
            favorites_count: self.favorites_count,
            tag_list: self.tag_list.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
