use actix_web::{http::StatusCode, HttpResponse};
use thiserror::Error;
use sqlx::Error as SqlxError;
use jsonwebtoken::errors::Error as JWTError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    DatabaseError,

    //#[error("Unauthorized")]
    //Unauthorized,

    #[error("Internal Server Error")]
    InternalServerError,

}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError => {
                HttpResponse::InternalServerError().json("Something went wrong with the database")
            }
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("Something went wrong, try again later")
            }
            //AppError::Unauthorized => {
            //    HttpResponse::Unauthorized().json("Unauthorized")
            //}
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            //AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(_err: SqlxError) -> Self {
        AppError::DatabaseError
    }
}

impl From<JWTError> for AppError {
    fn from(_err: JWTError) -> Self {
        AppError::InternalServerError
    }
}
