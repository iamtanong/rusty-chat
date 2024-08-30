use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

// For some reason, Normal Custom Error doesn't work and cause a lot of errors
// So I use this instead

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    BadRequest(String),
    Unathorized,
    NotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InternalServerError => write!(f, "Internal Server Error"),
            AppError::BadRequest(message) => write!(f, "Bad Request: {}", message),
            AppError::Unathorized => write!(f, "Unauthorized"),
            AppError::NotFound => write!(f, "Not Found"),
        }
    }
}

impl std::error::Error for AppError {}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_response = ErrorResponse {
            code: status.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unathorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

pub fn internal_error<E: std::fmt::Debug>(err: E) -> AppError {
    eprintln!("{:?}", err);
    AppError::InternalServerError
}
