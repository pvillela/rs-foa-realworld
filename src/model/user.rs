use chrono::{DateTime, Utc};

const PASSWORD_SALT_SIZE: u32 = 16;

pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String, // TODO: remove this field from code and databse
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
    pub fn create(username: String, email: String, password: String) -> User {
        let password_hash = todo!(); // = crypto.argon_password_hash(password)
        User {
            id: 0,
            username: username,
            email: email.to_lowercase(),
            password_hash,
            password_salt: "".to_string(),
            bio: String::from(""),
            image_link: "".to_string(),
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
            self.bio = String::from(bio);
        }
        if let Some(image_link) = v.image_link {
            self.image_link = image_link;
        }
    }

    pub fn to_profile(self, follows: bool) -> Profile {
        return Profile {
            user_id: self.id,
            username: self.username,
            bio: self.bio,
            image: self.image_link,
            following: follows,
        };
    }
}
