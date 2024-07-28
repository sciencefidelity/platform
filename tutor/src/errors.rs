use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl TutorError {
    fn error_response(&self) -> String {
        match self {
            Self::DBError(msg) => {
                println!("Database error occured: {msg:?}");
                "Database error".into()
            }
            Self::ActixError(msg) => {
                println!("Server error occured: {msg:?}");
                "Internal server error".into()
            }
            Self::NotFound(msg) => {
                println!("Not found error occured: {msg:?}");
                msg.into()
            }
        }
    }
}

impl error::ResponseError for TutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DBError(_) | Self::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}

// TODO: look into recursive impl.
#[allow(clippy::recursive_format_impl)]
impl fmt::Display for TutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl From<actix_web::error::Error> for TutorError {
    fn from(err: actix_web::error::Error) -> Self {
        Self::ActixError(err.to_string())
    }
}

impl From<SQLxError> for TutorError {
    fn from(err: SQLxError) -> Self {
        Self::DBError(err.to_string())
    }
}
