//! Error types for the hyw-embed crate.

use super::CyperError;
use thiserror::Error;

/// Possible errors when embedding text.
#[derive(Debug, Error)]
pub enum EmbedError {
    // HTTP client errors.
    /// Error when preparing the request.
    #[error("Failed to prepare request: {0}")]
    RequestPreparation(CyperError),
    /// Error when serializing the request body.
    #[error("Failed to serialize request body: {0}")]
    RequestSerialization(CyperError),
    /// Error when sending the request.
    #[error("Failed to send request: {0}")]
    RequestSend(CyperError),
    /// Error when parsing the response body.
    #[error("Failed to parse response body: {0}")]
    ResponseParse(CyperError),
    /// Error when decoding the base64 embedding.
    #[error("Failed to decode base64 embedding: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    // API errors.
    /// Invalid API key.
    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),
    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    /// Unknown API error.
    #[error("Unknown API error: {0}")]
    UnknownApiError(String),
}
