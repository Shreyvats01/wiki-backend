use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::common::response::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("{0}")]
    Failed(String),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
    #[error(transparent)]
    NotFound(#[from] NotFoundError),
}

#[derive(Debug, Error)]
pub enum NotFoundError {
    #[error("task don't found")]
    taskNotFound,
    #[error("User Not found")]
    UserNotFound,
    #[error("Tag aren't found")]
    TagNotFound,
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Room not found")]
    RoomNotFound,
    #[error("Daily progress room not found")]
    DailyProgressNotFound
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Message should not null")]
    InvalidMessage,
    #[error("task must be 5 cherecters long")]
    taskTooShort,
    #[error("Description must be 5 cherecters long")]
    DescriptionTooShort,
    #[error("Description cannot be null")]
    DescriptionCanNotBeNull,
    #[error("Tag must be 2 cherecter long or alphabets only")]
    InvalidTag,
    #[error("Category must be 2 cherecter long or alphabet only")]
    InvalidCategories,
    #[error("Invalid profile pic")]
    InvalidProfilePicUrl,
    #[error("user enter invalid email")]
    InvalidEmail,
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Name must be 3 cherecter long")]
    TooShortName,
    #[error("User already exits")]
    UserAlreadyExits,
    #[error("Failed to create token")]
    FailedToCreateToken,
    #[error("User profile is private!")]
    UnauthorizedAccess
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Db(error) => map_sqlx_error(error),
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::Failed(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };
        (
            status,
            Json(ApiResponse::<()>::error(message)),
        )
            .into_response()
    }
}

fn map_sqlx_error(error: sqlx::Error) -> (StatusCode, String) {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Record not found".into()),
        sqlx::Error::Database(db_error) => match db_error.code().as_deref() {
            Some("23505") => {
                let message = match db_error.constraint() {
                    Some("users_email_key") => "User already exits",
                    _ => "Resource already exits",
                };
                (StatusCode::CONFLICT, message.into())
            }
            Some("23503") => (StatusCode::BAD_REQUEST, "Invalid referance value".into()),
            Some("23502") => (StatusCode::BAD_REQUEST, "Missing required field".into()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
        },
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()),
    }
}
