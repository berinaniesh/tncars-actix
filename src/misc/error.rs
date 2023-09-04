use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::errors::Error as JWTError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Something went wrong, please try again later")]
    InternalServerError,
    #[error("Bad Request: (0)")]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try again later."),
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_err: sqlx::Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<JWTError> for AppError {
    fn from(err: JWTError) -> Self {
        AppError::InternalServerError
    }
}
