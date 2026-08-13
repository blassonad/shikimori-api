//! Типобезопасный асинхронный клиент официального Shikimori REST API v1/v2.
//!
//! Crate намеренно не реализует GraphQL и OAuth authorization/refresh flows. Для
//! защищённых REST-операций передайте уже полученный access token в
//! [`ClientConfig::access_token`].
//!
//! ## Быстрый старт
//!
//! ```no_run
//! use shikimori_api::{ClientConfig, ShikimoriClient};
//!
//! # async fn example() -> Result<(), shikimori_api::Error> {
//! let client = ShikimoriClient::new(ClientConfig::builder("my-shikimori-app/0.1").build()?);
//! let animes = client.list_animes(Default::default()).await?;
//! println!("received {} anime records", animes.len());
//! # Ok(())
//! # }
//! ```
//!
//! The official REST documentation imposes both 5 requests/second and 90
//! requests/minute. The default client limiter enforces both quotas before a
//! request is sent. See <https://shikimori.io/api/doc/1.0>.

#![forbid(unsafe_code)]

mod client;
mod encoding;
mod error;
mod params;
pub mod types;

pub use client::{ClientConfig, ClientConfigBuilder, ShikimoriClient};
pub use error::{ApiError, Error, Result};
pub use params::*;

/// Namespace containing all REST v1 operations.
pub mod v1;

/// Namespace containing all REST v2 operations.
pub mod v2;
