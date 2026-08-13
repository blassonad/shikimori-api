//! Общий Hyper/Rustls transport и builder клиента.

use std::{num::NonZeroU32, sync::Arc};

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use hyper::{
    body::to_bytes,
    client::HttpConnector,
    header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, RETRY_AFTER, USER_AGENT},
    Body, Client, Method, Request, StatusCode,
};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    encoding::query_string,
    error::{ApiError, Error, Result},
    params::QueryParameters,
};

const DEFAULT_BASE_URL: &str = "https://shikimori.io";
const DEFAULT_RPS: u32 = 5;
const DEFAULT_RPM: u32 = 90;

/// Immutable configuration used to construct a [`ShikimoriClient`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    /// API origin, without a trailing slash. Defaults to `https://shikimori.io`.
    pub base_url: String,
    /// Required meaningful application identifier sent as `User-Agent`.
    pub user_agent: String,
    /// Optional access token for protected REST routes. OAuth issuance/refresh is out of scope.
    pub access_token: Option<String>,
    /// Maximum requests per second. Defaults to Shikimori's documented 5 rps.
    pub requests_per_second: NonZeroU32,
    /// Maximum requests per minute. Defaults to Shikimori's documented 90 rpm.
    pub requests_per_minute: NonZeroU32,
}

impl ClientConfig {
    /// Starts a builder with a required application-specific user agent.
    pub fn builder(user_agent: impl Into<String>) -> ClientConfigBuilder {
        ClientConfigBuilder {
            base_url: DEFAULT_BASE_URL.to_owned(),
            user_agent: user_agent.into(),
            access_token: None,
            requests_per_second: NonZeroU32::new(DEFAULT_RPS).expect("non-zero default"),
            requests_per_minute: NonZeroU32::new(DEFAULT_RPM).expect("non-zero default"),
        }
    }
}

/// Fluent builder for [`ClientConfig`].
#[derive(Debug, Clone)]
pub struct ClientConfigBuilder {
    base_url: String,
    user_agent: String,
    access_token: Option<String>,
    requests_per_second: NonZeroU32,
    requests_per_minute: NonZeroU32,
}

impl ClientConfigBuilder {
    /// Overrides the API origin, useful for deterministic local tests.
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into();
        self
    }

    /// Sets an already-issued bearer access token. The token is never included in errors.
    pub fn access_token(mut self, value: impl Into<String>) -> Self {
        self.access_token = Some(value.into());
        self
    }

    /// Omits bearer authentication from all outgoing requests.
    pub fn without_access_token(mut self) -> Self {
        self.access_token = None;
        self
    }

    /// Changes the requests-per-second quota. Both quotas remain active.
    pub fn requests_per_second(mut self, value: NonZeroU32) -> Self {
        self.requests_per_second = value;
        self
    }

    /// Changes the requests-per-minute quota. Both quotas remain active.
    pub fn requests_per_minute(mut self, value: NonZeroU32) -> Self {
        self.requests_per_minute = value;
        self
    }

    /// Validates configuration and returns it.
    pub fn build(self) -> Result<ClientConfig> {
        let base_url = self.base_url.trim_end_matches('/').to_owned();
        if !(base_url.starts_with("https://") || base_url.starts_with("http://")) {
            return Err(Error::Configuration(
                "base_url must begin with http:// or https://".into(),
            ));
        }
        if self.user_agent.trim().is_empty() {
            return Err(Error::Configuration("user_agent must not be empty".into()));
        }
        self.user_agent
            .parse::<HeaderValue>()
            .map_err(|error| Error::Configuration(format!("invalid user_agent header: {error}")))?;
        if let Some(token) = &self.access_token {
            if token.trim().is_empty() {
                return Err(Error::Configuration(
                    "access_token must not be empty when supplied".into(),
                ));
            }
        }
        Ok(ClientConfig {
            base_url,
            user_agent: self.user_agent,
            access_token: self.access_token,
            requests_per_second: self.requests_per_second,
            requests_per_minute: self.requests_per_minute,
        })
    }
}

type HttpsClient = Client<HttpsConnector<HttpConnector>, Body>;

/// Asynchronous Shikimori REST client.
///
/// The type is cheap to clone; clones share the same transport and both global
/// governor rate limiters.
#[derive(Clone)]
pub struct ShikimoriClient {
    http: HttpsClient,
    base_url: Arc<str>,
    user_agent: HeaderValue,
    authorization: Option<HeaderValue>,
    per_second: Arc<DefaultDirectRateLimiter>,
    per_minute: Arc<DefaultDirectRateLimiter>,
}

impl std::fmt::Debug for ShikimoriClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ShikimoriClient")
            .field("base_url", &self.base_url)
            .field("has_access_token", &self.authorization.is_some())
            .finish_non_exhaustive()
    }
}

impl ShikimoriClient {
    /// Creates a client using HTTPS backed by Rustls/webpki roots.
    pub fn new(config: ClientConfig) -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();
        let authorization = config.access_token.map(|token| {
            HeaderValue::from_str(&format!("Bearer {token}"))
                .expect("token was validated as non-empty and header-safe by builder")
        });
        Self {
            http: Client::builder().build(connector),
            base_url: Arc::from(config.base_url),
            user_agent: HeaderValue::from_str(&config.user_agent)
                .expect("user agent was validated by builder"),
            authorization,
            per_second: Arc::new(RateLimiter::direct(Quota::per_second(
                config.requests_per_second,
            ))),
            per_minute: Arc::new(RateLimiter::direct(Quota::per_minute(
                config.requests_per_minute,
            ))),
        }
    }

    /// Returns the configured origin, mainly useful in diagnostics/tests.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Returns whether the client will send bearer authorization.
    pub fn has_access_token(&self) -> bool {
        self.authorization.is_some()
    }

    pub(crate) async fn get<T, Q>(&self, path: &str, query: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: QueryParameters,
    {
        self.request_json(Method::GET, path, query, None::<&()>)
            .await
    }

    pub(crate) async fn post_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_json(Method::POST, path, &(), Some(body)).await
    }

    pub(crate) async fn post_empty(&self, path: &str) -> Result<()> {
        self.request_unit(Method::POST, path, &(), None::<&()>)
            .await
    }

    pub(crate) async fn post_empty_json<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json(Method::POST, path, &(), None::<&()>)
            .await
    }

    pub(crate) async fn patch_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_json(Method::PATCH, path, &(), Some(body))
            .await
    }

    pub(crate) async fn put_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        self.request_json(Method::PUT, path, &(), Some(body)).await
    }

    pub(crate) async fn post_unit<B>(&self, path: &str, body: Option<&B>) -> Result<()>
    where
        B: Serialize,
    {
        self.request_unit(Method::POST, path, &(), body).await
    }

    pub(crate) async fn delete_json<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.request_json(Method::DELETE, path, &(), None::<&()>)
            .await
    }

    pub(crate) async fn delete_unit(&self, path: &str) -> Result<()> {
        self.request_unit(Method::DELETE, path, &(), None::<&()>)
            .await
    }

    pub(crate) async fn post_multipart<T>(
        &self,
        path: &str,
        body: Vec<u8>,
        content_type: String,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.execute_json(
            Method::POST,
            path,
            &(),
            Body::from(body),
            Some(content_type),
        )
        .await
    }

    pub(crate) async fn post_plain_text(&self, path: &str) -> Result<String> {
        let bytes = self
            .execute(Method::POST, path, &(), Body::empty(), None)
            .await?;
        String::from_utf8(bytes).map_err(|error| Error::UnexpectedBody(error.to_string()))
    }

    async fn request_json<T, Q, B>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        Q: QueryParameters,
        B: Serialize,
    {
        let (body, content_type) = match body {
            Some(body) => (
                Body::from(simd_json::serde::to_vec(body)?),
                Some("application/json".to_owned()),
            ),
            None => (Body::empty(), None),
        };
        self.execute_json(method, path, query, body, content_type)
            .await
    }

    async fn request_unit<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Option<&B>,
    ) -> Result<()>
    where
        Q: QueryParameters,
        B: Serialize,
    {
        let (body, content_type) = match body {
            Some(body) => (
                Body::from(simd_json::serde::to_vec(body)?),
                Some("application/json".to_owned()),
            ),
            None => (Body::empty(), None),
        };
        self.execute(method, path, query, body, content_type)
            .await
            .map(|_| ())
    }

    async fn execute_json<T, Q>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Body,
        content_type: Option<String>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        Q: QueryParameters,
    {
        let mut bytes = self
            .execute(method, path, query, body, content_type)
            .await?;
        if bytes.is_empty() {
            return Err(Error::UnexpectedBody(
                "expected JSON response but received an empty success body".into(),
            ));
        }
        Ok(simd_json::serde::from_slice(&mut bytes)?)
    }

    async fn execute<Q>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
        body: Body,
        content_type: Option<String>,
    ) -> Result<Vec<u8>>
    where
        Q: QueryParameters,
    {
        self.per_second.until_ready().await;
        self.per_minute.until_ready().await;

        let uri = self.uri(path, query)?;
        let mut builder = Request::builder().method(method).uri(uri);
        {
            let headers = builder.headers_mut().ok_or_else(|| {
                Error::InvalidRequest("request builder headers unavailable".into())
            })?;
            headers.insert(USER_AGENT, self.user_agent.clone());
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            if let Some(authorization) = &self.authorization {
                headers.insert(AUTHORIZATION, authorization.clone());
            }
            if let Some(content_type) = content_type {
                let value = HeaderValue::from_str(&content_type).map_err(|error| {
                    Error::InvalidRequest(format!("invalid content type: {error}"))
                })?;
                headers.insert(CONTENT_TYPE, value);
            }
        }
        let request = builder
            .body(body)
            .map_err(|error| Error::InvalidRequest(error.to_string()))?;
        let response = self.http.request(request).await.map_err(Error::Transport)?;
        let status = response.status();
        let retry_after_seconds = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok());
        let bytes = to_bytes(response.into_body())
            .await
            .map_err(Error::Body)?
            .to_vec();

        if !status.is_success() {
            return Err(Error::Api(api_error(status, retry_after_seconds, bytes)));
        }
        Ok(bytes)
    }

    fn uri<Q: QueryParameters>(&self, path: &str, query: &Q) -> Result<hyper::Uri> {
        if !path.starts_with('/') {
            return Err(Error::InvalidUri(
                "internal path must start with '/'".into(),
            ));
        }
        let pairs = query.pairs();
        let suffix = if pairs.is_empty() {
            String::new()
        } else {
            format!("?{}", query_string(&pairs))
        };
        format!("{}{}{}", self.base_url, path, suffix)
            .parse()
            .map_err(|error: hyper::http::uri::InvalidUri| Error::InvalidUri(error.to_string()))
    }
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    #[serde(default)]
    errors: Vec<String>,
}

fn api_error(status: StatusCode, retry_after_seconds: Option<u64>, mut bytes: Vec<u8>) -> ApiError {
    let body = String::from_utf8_lossy(&bytes).into_owned();
    let messages = simd_json::serde::from_slice::<ErrorBody>(&mut bytes)
        .map(|parsed| parsed.errors)
        .unwrap_or_default();
    ApiError {
        status: status.as_u16(),
        messages,
        body,
        retry_after_seconds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::{ListAnimesQuery, QueryParameters};

    #[test]
    fn builder_rejects_an_empty_user_agent() {
        assert!(ClientConfig::builder(" ").build().is_err());
    }

    #[test]
    fn builds_query_uri_without_url_dependency() {
        let config = ClientConfig::builder("test-app/1.0")
            .base_url("https://example.invalid/")
            .build()
            .unwrap();
        let client = ShikimoriClient::new(config);
        let query = ListAnimesQuery {
            search: Some("cowboy bebop".into()),
            ..Default::default()
        };
        assert_eq!(
            client.uri("/api/animes", &query).unwrap().to_string(),
            "https://example.invalid/api/animes?search=cowboy%20bebop"
        );
        assert_eq!(query.pairs().len(), 1);
    }
}
