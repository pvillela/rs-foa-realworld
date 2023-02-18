use crate::arch::crypto::argon_password_check;
use crate::common::AppError;
use crate::model::User;

pub type UserAuthenticateBfT = Box<dyn Fn(User, String) -> Result<(), AppError>>;

pub fn user_authenticate_bf(user: User, password: String) -> Result<(), AppError> {
    argon_password_check(password, user.password_hash)?;
    Ok(())
}

fn _type_check() -> UserAuthenticateBfT {
    Box::new(user_authenticate_bf)
}
