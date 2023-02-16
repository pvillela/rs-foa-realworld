use arcstr::ArcStr;
use chrono::{DateTime, Utc};

const PASSWORD_SALT_SIZE: u32 = 16;

pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub bio: ArcStr,
    pub image_link: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Profile {
    pub user_id: u64,
    pub username: String,
    pub bio: ArcStr,
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
    pub fn create(username: &str, email: &str, password: &str) -> User {
        let password_hash = todo!(); // = crypto.argon_password_hash(password)
        User {
            id: 0,
            username: username.to_owned(),
            email: email.to_lowercase(),
            password_hash,
            password_salt: "".to_owned(),
            bio: ArcStr::from(""),
            image_link: "".to_owned(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn update(&mut self, v: UserPatch) {
        if let Some(username) = v.username {
            self.username = username;
        }
        if let Some(email) = v.email {
            self.email = email.to_lowercase();
        }
        if let Some(password) = v.password {
            self.password_salt = todo!(); //  crypto.random_string(password_salt_size)
            self.password_hash = todo!(); //  crypto.argon_password_hash(password + self.password_salt)
        }
        if let Some(bio) = v.bio {
            self.bio = ArcStr::from(bio);
        }
        if let Some(image_link) = v.image_link {
            self.image_link = image_link;
        }
    }

    pub fn to_profile(user: &User, follows: bool) -> Profile {
        return Profile {
            user_id: user.id,
            username: user.username.clone(),
            bio: user.bio.clone(),
            image: user.image_link.clone(),
            following: follows,
        };
    }
}
