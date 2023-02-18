use crate::arch::crypto::argon_password_hash;
use crate::common::AppError;
use chrono::{DateTime, Utc};

pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub bio: String,
    pub image_link: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Profile {
    pub user_id: u64,
    pub username: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}

pub struct UserPatch {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image_link: Option<String>,
}

impl User {
    pub fn create(username: String, email: String, password: String) -> Result<User, AppError> {
        let password_hash = argon_password_hash(password)?;
        Ok(User {
            id: 0,
            username,
            email: email.to_lowercase(),
            password_hash,
            bio: String::from(""),
            image_link: "".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn update(&mut self, v: UserPatch) -> Result<(), AppError> {
        if let Some(username) = v.username {
            self.username = username;
        }
        if let Some(email) = v.email {
            self.email = email.to_lowercase();
        }
        if let Some(password) = v.password {
            self.password_hash = argon_password_hash(password)?;
        }
        if let Some(bio) = v.bio {
            self.bio = String::from(bio);
        }
        if let Some(image_link) = v.image_link {
            self.image_link = image_link;
        }
        Ok(())
    }
}

impl Profile {
    pub fn from_user(user: User, follows: bool) -> Profile {
        Profile {
            user_id: user.id,
            username: user.username,
            bio: user.bio,
            image: user.image_link,
            following: follows,
        }
    }
}
