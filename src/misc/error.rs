use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
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
    fn from(err: sqlx::Error) -> Self {
        eprintln!("{:?}", err);
        AppError::InternalServerError
    }
}
