use actix_web::{http::StatusCode, HttpResponse};
use jsonwebtoken::errors::Error as JWTError;
use serde_json::json;
use serde_json::Value as JsonValue;
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError(JsonValue),

    #[error("Not Found")]
    NotFound(JsonValue),

    #[error("Unauthorized")]
    Unauthorized(JsonValue),

    #[error("Unprocessable Entity")]
    UnprocessableEntity(JsonValue),
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError(ref msg) => HttpResponse::InternalServerError().json(msg),
            AppError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => {
                AppError::NotFound(json!({"message": "The requested resource was not found"}))
            }
            SqlxError::Database(_) => AppError::UnprocessableEntity(
                json!({"message": "You probably are violating some constraints"}),
            ),
            _ => AppError::InternalServerError(
                json!({"message": "Something went wrong with the database"}),
            ),
        }
    }
}

impl From<JWTError> for AppError {
    fn from(_err: JWTError) -> Self {
        AppError::Unauthorized(json!({"message": "Something wrong with JWT"}))
    }
}
