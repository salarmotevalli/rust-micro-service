use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use backtrace::Backtrace;
use serde_json::Value;
use thiserror::Error;

/// The Standard Error for most of Merino
pub struct HandlerError {
    // Important: please make sure to update the implementation of
    // std::fmt::Debug for this struct if new fields are added here.
    /// The wrapped error value.
    kind: HandlerErrorKind,
    /// The backtrace related to the wrapped error.
    pub(crate) backtrace: Backtrace,
}

/// An error that happened in a web handler.
#[derive(Error, Debug)]
pub enum HandlerErrorKind {
    /// A generic error, when there is nothing more specific to say.
    #[error("Internal error {0}")]
    Internal(String),
}

impl From<mongodb::bson::oid::Error> for HandlerErrorKind {
    fn from(value: mongodb::bson::oid::Error) -> Self {
        HandlerErrorKind::Internal(value.to_string())
    }
}

impl From<mongodb::error::Error> for HandlerErrorKind {
    fn from(value: mongodb::error::Error) -> Self {
        HandlerErrorKind::Internal(value.to_string())
    }
}

impl From<HandlerErrorKind> for actix_web::Error {
    fn from(kind: HandlerErrorKind) -> Self {
        let error: HandlerError = kind.into();
        error.into()
    }
}

impl HandlerError {
    /// Access the wrapped error.
    pub fn kind(&self) -> &HandlerErrorKind {
        &self.kind
    }

    /// Get an `HandlerError` representing an `Internal` error.
    ///
    /// This is a convenience function: the same result can be
    /// achieved by directly using `HandlerErrorKind::Internal.into()`.
    pub fn internal() -> Self {
        HandlerErrorKind::Internal(String::from("")).into()
    }
}

impl Error for HandlerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }
}

impl<T> From<T> for HandlerError
where
    HandlerErrorKind: From<T>,
{
    fn from(item: T) -> Self {
        HandlerError {
            kind: HandlerErrorKind::from(item),
            backtrace: Backtrace::new(),
        }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl std::fmt::Debug for HandlerError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Sentry will scan the printed debug information for `HandlerError`
        // to determine the "event type" to display and to group events by:
        // to make sure different errors don't get grouped together, we format
        // the name of this debug struct as `HandlerError/<error name>`.
        // See `sentry::parse_type_from_debug` used by middleware/sentry.rs
        fmt.debug_struct(&format!("HandlerError/{:?}", &self.kind))
            .field("kind", &self.kind)
            .field("backtrace", &self.backtrace)
            .finish()
    }
}

impl ResponseError for HandlerError {
    /// Convert the error to an HTTP status code.
    fn status_code(&self) -> StatusCode {
        match self.kind() {
            HandlerErrorKind::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut response = HashMap::new();
        response.insert(
            "error".to_owned(),
            Value::String(format!("{}", self.kind())),
        );
        HttpResponse::InternalServerError().json(response)
    }
}
