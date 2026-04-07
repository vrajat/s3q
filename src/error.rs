use std::{error::Error as StdError, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    NotImplemented(&'static str),
    QueueNotFound(String),
    MessageNotFound(i64),
    LeaseExpired,
    OwnershipMismatch,
    InvalidArgument(String),
    StoreUnavailable(String),
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotImplemented(message) => write!(f, "not implemented: {message}"),
            Self::QueueNotFound(queue) => write!(f, "queue not found: {queue}"),
            Self::MessageNotFound(message_id) => write!(f, "message not found: {message_id}"),
            Self::LeaseExpired => write!(f, "message lease expired"),
            Self::OwnershipMismatch => write!(f, "message is owned by another consumer"),
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::StoreUnavailable(message) => write!(f, "store unavailable: {message}"),
            Self::Internal(message) => write!(f, "internal error: {message}"),
        }
    }
}

impl StdError for Error {}

pub type Result<T> = std::result::Result<T, Error>;
