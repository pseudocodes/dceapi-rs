//! Error types for the DCE API client.
//!
//! This module provides error handling following the API specification:
//! - 200: Success
//! - 400: Parameter error
//! - 401: Permission denied
//! - 402: Token expired
//! - 500: Server error
//! - 501: Rate limit

use thiserror::Error;

/// API error codes as defined by the DCE API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    /// 200: Success
    Success = 200,
    /// 400: Parameter error
    ParamError = 400,
    /// 401: Permission denied
    NoPermission = 401,
    /// 402: Token expired
    TokenExpired = 402,
    /// 500: Internal server error
    ServerError = 500,
    /// 501: Rate limit exceeded
    RateLimit = 501,
}

impl ErrorCode {
    /// Create ErrorCode from i32 value.
    pub fn from_code(code: i32) -> Option<ErrorCode> {
        match code {
            200 => Some(ErrorCode::Success),
            400 => Some(ErrorCode::ParamError),
            401 => Some(ErrorCode::NoPermission),
            402 => Some(ErrorCode::TokenExpired),
            500 => Some(ErrorCode::ServerError),
            501 => Some(ErrorCode::RateLimit),
            _ => None,
        }
    }
}

/// The main error type for the DCE API client.
#[derive(Error, Debug)]
pub enum Error {
    /// API returned an error response.
    #[error("API error {code}: {message}")]
    Api {
        /// The error code from the API.
        code: i32,
        /// The error message from the API.
        message: String,
    },

    /// Authentication failed.
    #[error("authentication error: {reason}")]
    Auth {
        /// The reason for the authentication failure.
        reason: String,
    },

    /// Network or HTTP error.
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Validation error for request parameters.
    #[error("validation error on field '{field}': {message}")]
    Validation {
        /// The field that failed validation.
        field: String,
        /// The validation error message.
        message: String,
    },

    /// JSON parsing error.
    #[error("parse error: {err}, raw response: {raw_response}")]
    Parse {
        /// The raw response that failed to parse.
        raw_response: String,
        /// The parsing error.
        err: String,
    },
}

impl Error {
    /// Create a new API error.
    pub fn api(code: i32, message: impl Into<String>) -> Self {
        Error::Api {
            code,
            message: message.into(),
        }
    }

    /// Create a new authentication error.
    pub fn auth(reason: impl Into<String>) -> Self {
        Error::Auth {
            reason: reason.into(),
        }
    }

    /// Create a new validation error.
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Error::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new parse error.
    pub fn parse(raw_response: impl Into<String>, err: impl Into<String>) -> Self {
        Error::Parse {
            raw_response: raw_response.into(),
            err: err.into(),
        }
    }

    /// Check if this is a token expired error.
    pub fn is_token_expired(&self) -> bool {
        matches!(self, Error::Api { code, .. } if *code == ErrorCode::TokenExpired as i32)
    }

    /// Get the error code if this is an API error.
    pub fn error_code(&self) -> Option<ErrorCode> {
        if let Error::Api { code, .. } = self {
            ErrorCode::from_code(*code)
        } else {
            None
        }
    }
}

/// Result type alias for DCE API operations.
pub type Result<T> = std::result::Result<T, Error>;
