use crate::model::User;

pub struct UserOut {
    pub user: UserOut0,
}

pub struct UserOut0 {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: String,
}

impl UserOut {
    pub fn from_model(user: User, token: String) -> UserOut {
        UserOut {
            user: UserOut0 {
                email: user.email,
                token,
                username: user.username,
                bio: user.bio,
                image: user.image_link,
            },
        }
    }
}
