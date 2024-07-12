use crate::arch::di::CfgDeps;
use crate::common::AppError;
use crate::model::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

pub type UserGenTokenBfT = fn(User) -> Result<String, AppError>;

pub struct UserGenTokenHmacBfCfgInfo {
    key: &'static [u8],
    token_time_to_live: Duration,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    sub: String,
    exp: i64,
}

pub fn user_gen_token_hmac_bf(user: User) -> Result<String, AppError> {
    if user.username == "" {
        return Err(AppError::UsernameEmpty);
    }

    let cfg = USER_GEN_TOKEN_BF_CFG.get_cfg();
    let claims = Claims {
        iss: "rs-foa-realworld".to_owned(),
        sub: user.username,
        exp: Utc::now().timestamp() + cfg.token_time_to_live.num_seconds(),
    };

    encode(
        &Header::default(), // HS256
        &claims,
        &EncodingKey::from_secret(cfg.key),
    )
    .map_err(|err| err.into())
}

pub static USER_GEN_TOKEN_BF_CFG: CfgDeps<UserGenTokenHmacBfCfgInfo, ()> = CfgDeps::new();
