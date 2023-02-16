use crate::arch::util;
use crate::model::{Profile, User};
use chrono::{DateTime, Utc};

pub struct Article {
    pub id: u64,
    pub author_id: u64,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub body: String,
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
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i64,
}

pub struct ArticlePatch {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

impl Article {
    pub fn create(
        author: User,
        title: String,
        description: String,
        body: String,
        tag_list: Vec<String>,
    ) -> Article {
        Article {
            id: 0,
            author_id: author.id,
            title,
            slug: util::slug(&title), // make sure this is unique index in database
            description: String::from(description),
            body: String::from(body),
            favorites_count: 0,
            tag_list,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Updates the receiver but does not change the slug when the title changes.
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

    pub fn to_article_plus(self, favorited: bool, author: Profile) -> ArticlePlus {
        return ArticlePlus {
            id: self.id,
            slug: self.slug,
            author: Profile {
                user_id: author.user_id,
                username: author.username,
                bio: author.bio,
                image: author.image,
                following: author.following,
            },
            title: self.title,
            description: self.description,
            body: self.body,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited,
            favorites_count: self.favorites_count,
        };
    }
}

impl ArticlePlus {
    pub fn to_article(self) -> Article {
        Article {
            id: self.id,
            author_id: self.author.user_id,
            title: self.title,
            slug: self.slug,
            description: self.description,
            body: self.body,
            favorites_count: self.favorites_count,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
