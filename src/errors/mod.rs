use actix::MailboxError;
use actix_http;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::r2d2::PoolError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use std::convert::From;
use uuid::ParseError;
#[derive(Debug, Display,PartialEq)]
pub enum ServiceError {
    // 400
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    // 401
    #[display(fmt = "Unauthorized")]
    Unauthorized,

    // 404
    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    // 500+
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

// impl ResponseError trait allows to convert errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("4444444444444444444444444444")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
        }
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let msg = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(msg);
                }
                ServiceError::BadRequest(
                    info.details().unwrap_or_else(|| info.message()).to_string(),
                )
            }
            DieselError::NotFound => {
                ServiceError::NotFound("requested record was not found".into())
            }
            _ => ServiceError::NotFound("requested record was not found".into()),
        }
    }
}

impl From<PoolError> for ServiceError {
    fn from(_error: PoolError) -> Self {
        //ServiceError::InternalServerError
        ServiceError::NotFound("222222222222222222222222".into())
    }
}

impl From<MailboxError> for ServiceError {
    fn from(_error: MailboxError) -> Self {
        //ServiceError::InternalServerError
        ServiceError::NotFound("11111111111111111111111111111111".into())
    }
}

impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}
impl From<actix_http::error::Error> for ServiceError {
    fn from(_: actix_http::error::Error) -> Self {
        ServiceError::BadRequest("actix_http::error::Error".into())
    }
}
impl From<actix_http::error::PayloadError> for ServiceError {
    fn from(_: actix_http::error::PayloadError) -> Self {
        ServiceError::BadRequest("actix_http::error::PayloadError".into())
    }
}

/*
impl From<actix_http::client::error::SendRequestError> for ServiceError {
    fn from(_: actix_http::client::error::SendRequestError) -> Self {
        ServiceError::BadRequest("actix_http::client::error::SendRequestError".into())
    }
}
*/
