use crate::common::AppError;
use crate::model::User;

pub struct UserRegisterIn {
    pub user: UserRegisterIn0,
}

pub struct UserRegisterIn0 {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserRegisterIn {
    pub fn to_user(self) -> Result<User, AppError> {
        User::create(self.user.username, self.user.email, self.user.password)
    }
}
