use argon2::{
    password_hash::{
        errors::Error as PhError, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Algorithm::Argon2i,
    Argon2, Params, Version,
};
use once_cell::sync::Lazy;
use thiserror::Error;

const IS_DEV: bool = true;

static ARGON2: Lazy<Argon2> = Lazy::new(|| {
    let dev_params = Params::new(10 * 1024, 1, 1, Some(Params::DEFAULT_OUTPUT_LEN))
        .expect("Error defining Argon2 params for dev");
    // salt_length:	16,
    // key_length:	32,

    let prod_params = Params::new(1 * 1024 * 1024, 1, 3, Some(Params::DEFAULT_OUTPUT_LEN))
        .expect("Error defining Argon2 params for prod");
    // 	salt_length:	8,
    // 	key_length:	8,

    let argon_params = if IS_DEV { dev_params } else { prod_params };

    Argon2::new(Argon2i, Version::V0x13, argon_params)
});

pub fn argon_password_hash(password: String) -> Result<String, PasswordHashError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash_str = ARGON2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash_str)
}

pub fn argon_password_check(password: String, hash: String) -> Result<(), PasswordHashError> {
    let parsed_hash = PasswordHash::new(&hash)?;
    ARGON2.verify_password(password.as_bytes(), &parsed_hash)?;
    Ok(())
}

#[derive(Error, Debug)]
#[error("password hash error: \"{0}\"")]
pub struct PasswordHashError(PhError);

impl From<PhError> for PasswordHashError {
    fn from(e: PhError) -> Self {
        PasswordHashError(e)
    }
}
