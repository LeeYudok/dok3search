//! Shared error definitions for dok3search crates.

use thiserror::Error;

/// Domain and infrastructure boundary errors mapped consistently across API and adapters.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Dok3Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("index error: {0}")]
    Index(String),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl Dok3Error {
    /// Returns a stable uppercase error code suitable for API and audit logs.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::Parse(_) => "PARSE_ERROR",
            Self::Index(_) => "INDEX_ERROR",
            Self::Storage(_) => "STORAGE_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dok3Error;

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(Dok3Error::InvalidInput("x".into()).code(), "INVALID_INPUT");
        assert_eq!(Dok3Error::NotFound("x".into()).code(), "NOT_FOUND");
        assert_eq!(Dok3Error::Forbidden("x".into()).code(), "FORBIDDEN");
        assert_eq!(Dok3Error::Parse("x".into()).code(), "PARSE_ERROR");
        assert_eq!(Dok3Error::Index("x".into()).code(), "INDEX_ERROR");
        assert_eq!(Dok3Error::Storage("x".into()).code(), "STORAGE_ERROR");
        assert_eq!(Dok3Error::Internal("x".into()).code(), "INTERNAL_ERROR");
    }
}
