#![doc = include_str!("docs/crate-overview.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]

mod client;
mod encoding;
mod error;
/// Типизированные query, request body и input-enum Shikimori REST API.
mod params;
/// Типизированные response-модели и общие ресурсы Shikimori REST API.
pub mod types;

pub use client::{ClientConfig, ClientConfigBuilder, ShikimoriClient};
pub use error::{ApiError, Error, Result};
pub use params::*;

/// Namespace containing all REST v1 operations.
pub mod v1;

/// Namespace containing all REST v2 operations.
pub mod v2;
