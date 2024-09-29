use crate::arch::{crypto::PasswordHashError, tx::DbErr};
use axum::response::{IntoResponse, Response};
use serde::{ser::SerializeStructVariant, Serialize};
use std::{error::Error as StdError, fmt::Debug};
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[non_exhaustive]
pub enum CoreAppError {
    #[error("duplicate article slug \"{0}\"")]
    DuplicateArticleSlug(String),

    #[error("duplicate article id {0}")]
    DuplicateArticleId(usize),

    #[error("article slug \"{0}\" not found")]
    ArticleSlugNotFound(String),

    #[error("article not found for id {id}")]
    ArticleNotFound { id: usize },

    #[error("article has missing fields for create operation")]
    ArticleCreateMissingFields,

    #[error("article with id \"{id}\" has already been favoriated")]
    ArticleAlreadyFavorited { id: usize },

    #[error("article with id \"{id}\" was not favorited")]
    ArticleWasNotFavorited { id: usize },

    #[error("comment not found because comment id not valid or user did not author the article")]
    CommentNotFound,

    #[error("profile not found")]
    ProfileNotFound,

    #[error("tag name {name} already exists")]
    TagNameAlreadyExists { name: String },

    #[error("tag with name {name} already exists on article with slug {slug}")]
    TagOnArticleAlreadyExists { name: String, slug: String },

    #[error("user not found for email {email}")]
    UserEmailNotFound { email: String },

    #[error("user with name \"{username}\" or email \"{email}\" already exists")]
    UsernameOrEmailDuplicate { username: String, email: String },

    #[error("user with name \"{0}\" already exists")]
    UsernameDuplicate(String),

    #[error("username empty")]
    UsernameEmpty,

    #[error("user not found for username \"{0}\"")]
    UsernameNotFound(String),

    #[error("user with username \"{username}\" was already followed")]
    UserAlreadyFollowed { username: String },

    #[error("user with username \"{username}\" was not followed")]
    UserWasNotFollowed { username: String },

    #[error("user \"{username}\" not authorized to take this action")]
    UnauthorizedUser { username: String },

    #[error("user authentication failed with name \"{username}\" and password \"{password}\"")]
    AuthenticationFailed { username: String, password: String },

    #[error("user not authenticated")]
    NotAuthenticated,

    #[error("user with email {0} already exists")]
    DuplicateUserEmail(String),

    #[error("validation failed: \"{msg}\"")]
    ValidationFailed { msg: String },
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Core(CoreAppError),

    #[error("library error due to: [{0}]")]
    LibraryErrorStr(String),

    #[error("library error due to: [{source}]")]
    LibraryError { source: Box<dyn SerError> },
}

impl AppError {
    /// The `cause`'s `to_string()` value is wrapped in an [`AppError::LibraryErrorStr`]
    fn library_error_str<T: StdError>(cause: &T) -> AppError {
        Self::LibraryErrorStr(cause.to_string())
    }
}

pub trait SerError: StdError {
    fn json_string(&self) -> String;
}

impl StdError for Box<dyn SerError> {}

impl<T> SerError for T
where
    T: StdError + Serialize,
{
    fn json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or(self.to_string())
    }
}

impl Serialize for Box<dyn SerError> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.json_string())
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AppError::Core(core_err) => core_err.serialize(serializer),
            AppError::LibraryErrorStr(s) => {
                serializer.serialize_newtype_variant("AppError", 1, "LibraryError", s)
            }
            AppError::LibraryError { source } => {
                let mut state =
                    serializer.serialize_struct_variant("AppError", 2, "LibraryError", 1)?;
                state.serialize_field("source", source)?;
                state.end()
            }
        }
    }
}

impl From<PasswordHashError> for AppError {
    fn from(e: PasswordHashError) -> Self {
        Self::library_error_str(&e)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::library_error_str(&e)
    }
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        Self::library_error_str(&e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
