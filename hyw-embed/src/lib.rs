//! Helper for creating embeddings using the Silicon Flow API.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

mod json;

use base64::{Engine as _, engine::general_purpose::STANDARD as DECODER};
use json::{RequestBody, ResponseBody};
use nyquest::{AsyncClient as Client, Body, ClientBuilder, Request};

// const API_ENDPOINT: &str = "https://api.siliconflow.com/v1/embeddings";
const API_ENDPOINT: &str = "https://api.siliconflow.cn/v1/embeddings";

/// A client for the Silicon Flow API.
#[derive(Debug, Clone)]
pub struct ApiClient {
    /// HTTP client.
    client: Client,
}

impl ApiClient {
    /// Create a new API client.
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client fails to build.
    // TODO: Handle errors gracefully.
    pub async fn new(api_key: &str) -> Self {
        let client = ClientBuilder::default()
            .with_header("Authorization", format!("Bearer {api_key}"))
            .build_async()
            .await
            .expect("Failed to build HTTP client");
        Self { client }
    }

    /// Embed text using the Silicon Flow API.
    ///
    /// # Panics
    ///
    /// Panics if the request fails or the response cannot be parsed.
    // TODO: Handle errors gracefully.
    pub async fn embed_text(&self, input: &[&str]) -> Vec<[f32; 1024]> {
        let body = RequestBody {
            model: "BAAI/bge-large-zh-v1.5",
            input,
            encoding_format: "base64",
        };
        let request = Request::post(API_ENDPOINT)
            .with_body(Body::json(&body).expect("Failed to serialize request body"));
        let response = self.client.request(request).await.expect("Request failed");
        let response_body: ResponseBody = response
            .json()
            .await
            .expect("Failed to parse response body");

        response_body
            .data
            .into_iter()
            .map(|data| {
                let bytes = DECODER
                    .decode(data.embedding.as_bytes())
                    .expect("Failed to decode base64 embedding");
                let mut embedding = [0.0; 1024];
                bytes.chunks_exact(4).enumerate().for_each(|(i, chunk)| {
                    embedding[i] = f32::from_le_bytes(
                        chunk.try_into().expect("Failed to convert bytes to f32"),
                    );
                });
                embedding
            })
            .collect()
    }
}
