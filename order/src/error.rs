use std::error::Error;
use std::fmt;

use backtrace::Backtrace;
use thiserror::Error;
use tonic::Status;

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

impl From<HandlerError> for Status {
    fn from(value: HandlerError) -> Self {
        Status::new(500.into(), value.kind().to_string())
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

