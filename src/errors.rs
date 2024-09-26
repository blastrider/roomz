use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,
    #[display("BadRequest: {}", _0)]
    BadRequest(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().finish(),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().body(message.clone())
            }
        }
    }
}
