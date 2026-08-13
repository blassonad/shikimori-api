//! Ошибки клиента и ошибки, возвращённые Shikimori API.

use std::{error::Error as StdError, fmt};

/// Convenience result type used by all public client methods.
pub type Result<T> = std::result::Result<T, Error>;

/// Structured error body returned by the Shikimori API when one is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    /// HTTP status code.
    pub status: u16,
    /// Optional error messages parsed from an `errors` array/string response.
    pub messages: Vec<String>,
    /// Server response body retained as UTF-8-lossy text for diagnostics.
    pub body: String,
    /// Server-provided retry delay in seconds, parsed from `Retry-After` if present.
    pub retry_after_seconds: Option<u64>,
}

/// Error returned by the client.
#[derive(Debug)]
pub enum Error {
    /// The client configuration or a request URI/header was invalid.
    Configuration(String),
    /// A percent-encoded URI could not be parsed by Hyper.
    InvalidUri(String),
    /// A request could not be created due to an invalid HTTP header/value.
    InvalidRequest(String),
    /// HTTPS transport failure from Hyper/Rustls.
    Transport(hyper::Error),
    /// Response body streaming failure from Hyper.
    Body(hyper::Error),
    /// JSON encoding or decoding failure from simd-json.
    Json(simd_json::Error),
    /// The configured rate limiter was closed or rejected the request.
    RateLimit(String),
    /// An HTTP success response unexpectedly had a non-JSON body.
    UnexpectedBody(String),
    /// Non-success HTTP response from Shikimori.
    Api(ApiError),
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Shikimori API returned HTTP {}", self.status)?;
        if !self.messages.is_empty() {
            write!(formatter, ": {}", self.messages.join("; "))?;
        }
        Ok(())
    }
}

impl StdError for ApiError {}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) => {
                write!(formatter, "invalid client configuration: {message}")
            }
            Self::InvalidUri(message) => write!(formatter, "invalid request URI: {message}"),
            Self::InvalidRequest(message) => write!(formatter, "invalid request: {message}"),
            Self::Transport(error) => write!(formatter, "HTTPS transport error: {error}"),
            Self::Body(error) => write!(formatter, "HTTP response body error: {error}"),
            Self::Json(error) => write!(formatter, "JSON codec error: {error}"),
            Self::RateLimit(message) => write!(formatter, "rate limiter error: {message}"),
            Self::UnexpectedBody(message) => {
                write!(formatter, "unexpected response body: {message}")
            }
            Self::Api(error) => error.fmt(formatter),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Transport(error) | Self::Body(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Api(error) => Some(error),
            _ => None,
        }
    }
}

impl From<simd_json::Error> for Error {
    fn from(error: simd_json::Error) -> Self {
        Self::Json(error)
    }
}
