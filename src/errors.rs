use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("BadRequest: {}", _0)]
    BadRequest(String),

    #[display("Not Found")]
    NotFound,

    #[display("Conflict: {}", _0)]
    Conflict(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().body("Internal Server Error".to_string())
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().body(message.clone())
            }
            ServiceError::NotFound => {
                HttpResponse::NotFound().body("Resource Not Found".to_string())
            }
            ServiceError::Conflict(ref message) => HttpResponse::Conflict().body(message.clone()),
        }
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        match error {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                ServiceError::Conflict(info.message().to_string())
            }
            DieselError::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError,
        }
    }
}
