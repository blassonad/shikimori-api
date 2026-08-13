//! Ошибки клиента и ошибки, возвращённые Shikimori API.

use std::{error::Error as StdError, fmt};

/// Стандартный результат всех публичных методов клиента.
///
/// Успешное значение содержит тип конкретного REST-ответа; ошибка — [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Структурированная не-2xx ошибка, возвращённая Shikimori API.
///
/// Доступна как [`Error::Api`]. Даже если server не вернул JSON-`errors`,
/// [`Self::body`] остаётся доступным для диагностики.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    /// HTTP status code неуспешного ответа.
    pub status: u16,
    /// Сообщения из server-side JSON поля `errors`; пустой вектор, если body имеет другой формат.
    pub messages: Vec<String>,
    /// Server response body как lossy UTF-8 text для безопасной диагностики.
    pub body: String,
    /// Числовая задержка в секундах из `Retry-After`, если header присутствует и парсится.
    pub retry_after_seconds: Option<u64>,
}

/// Ошибка, возвращаемая клиентом.
///
/// Transport/codec configuration errors не повторяются автоматически. Для
/// HTTP non-success используйте [`Self::Api`] и inspect [`ApiError::status`],
/// `messages` и `retry_after_seconds`.
#[derive(Debug)]
pub enum Error {
    /// Некорректная client configuration: origin, token или User-Agent.
    Configuration(String),
    /// Percent-encoded URI не удалось распарсить Hyper-ом.
    InvalidUri(String),
    /// Невозможно создать request из-за некорректного HTTP header/value.
    InvalidRequest(String),
    /// Ошибка HTTPS transport, полученная от Hyper/Rustls.
    Transport(hyper::Error),
    /// Ошибка streaming response body, полученная от Hyper.
    Body(hyper::Error),
    /// Ошибка JSON encode/decode, полученная от `simd_json`.
    Json(simd_json::Error),
    /// Внутренний limiter был закрыт либо отказал запросу.
    RateLimit(String),
    /// Успешный response имел пустое или не-JSON body там, где ожидается JSON model.
    UnexpectedBody(String),
    /// Неуспешный HTTP response Shikimori с сохранёнными diagnostic details.
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
