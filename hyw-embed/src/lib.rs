//! Helper for creating embeddings using the Silicon Flow API.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Fucking windows.")]

mod error;
mod json;

use base64::{Engine as _, engine::general_purpose::STANDARD as DECODER};
use cyper::{Client, Error as CyperError};
pub use error::EmbedError;
use http::{HeaderMap, StatusCode, header::InvalidHeaderValue};
use json::{RequestBody, ResponseBody};

// const API_ENDPOINT: &str = "https://api.siliconflow.com/v1/embeddings";
const API_ENDPOINT: &str = "https://api.siliconflow.cn/v1/embeddings";

/// The embedding type.
pub type Embedding = [f32; 1024];

/// A client for the Silicon Flow API.
#[derive(Debug, Clone)]
pub struct ApiClient {
    /// HTTP client.
    client: Client,
}

impl ApiClient {
    /// Create a new API client.
    ///
    /// # Errors
    ///
    /// Returns a [`InvalidHeaderValue`] if the API key does not make a valid header value.
    pub fn new(api_key: &str) -> Result<Self, InvalidHeaderValue> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {api_key}").parse()?);
        let client = Client::builder().default_headers(headers).build();

        Ok(Self { client })
    }

    /// Embed text using the Silicon Flow API.
    ///
    /// # Errors
    ///
    /// Returns an [`EmbedError`] if the request fails or the response cannot be parsed.
    ///
    /// # Panics
    ///
    /// This function should not panic under normal circumstances. If it does, then `chunks_exact` does not properly return a chunk of 4 bytes.
    pub async fn embed_text(&self, input: &[&str]) -> Result<Vec<Embedding>, EmbedError> {
        let body = RequestBody {
            model: "BAAI/bge-large-zh-v1.5",
            input,
            encoding_format: "base64",
        };
        let response = self
            .client
            .post(API_ENDPOINT)
            .map_err(EmbedError::RequestPreparation)?
            .json(&body)
            .map_err(EmbedError::RequestSerialization)?
            .send()
            .await
            .map_err(EmbedError::RequestSend)?;

        match response.status() {
            StatusCode::UNAUTHORIZED => {
                let error_text = response.text().await.unwrap_or_default();
                return Err(EmbedError::InvalidApiKey(error_text));
            }
            StatusCode::TOO_MANY_REQUESTS => {
                let error_text = response.text().await.unwrap_or_default();
                return Err(EmbedError::RateLimitExceeded(error_text));
            }
            StatusCode::OK => {}
            _ => {
                let error_text = response.text().await.unwrap_or_default();
                return Err(EmbedError::UnknownApiError(error_text));
            }
        }

        let response_body: ResponseBody = response
            .json()
            .await
            .map_err(EmbedError::ResponseParse)?;

        let result = response_body
            .data
            .into_iter()
            .map(|data| -> Result<Embedding, EmbedError> {
                let bytes = DECODER
                    .decode(data.embedding.as_bytes())?;
                let mut embedding = [0.0; 1024];
                bytes.chunks_exact(4).enumerate().for_each(|(i, chunk)| {
                    embedding[i] = f32::from_le_bytes(
                        chunk.try_into().expect("The chunk length should be 4 bytes"),
                    );
                });
                Ok(embedding)
            })
            .collect::<Result<_, _>>()?;

        Ok(result)
    }
}
