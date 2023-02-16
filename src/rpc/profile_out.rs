use crate::model::{Profile, User};

pub struct ProfileOut {
    pub profile: Profile,
}

impl ProfileOut {
    pub fn from_model(user: User, follows: bool) -> ProfileOut {
        Profile::from_user(user, follows)
    }
}
