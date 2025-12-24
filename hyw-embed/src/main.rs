#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Fucking windows.")]

use hyw_base::Hyw;
use hyw_embed::ApiClient;
use std::io::Write;

// const BATCH_SIZE: usize = 128;
const BATCH_SIZE: usize = 16; // DEBUG

#[compio::main]
async fn main() -> Result<(), hyw_embed::EmbedError> {
    let api_key = std::env::var("SILICON_FLOW_API_KEY").expect("SILICON_FLOW_API_KEY not set");
    let client = ApiClient::new(&api_key).expect("Failed to create API client");
    let hyw_iter = Hyw::all();
    let size = hyw_iter.size_hint().0;

    // Lock stdout for better performance
    let mut lock = std::io::stdout().lock();
    writeln!(lock, "Total hyw count: {size}").unwrap();

    // DEBUG: Process only first 2 batches
    for batch in itermore::IterArrayChunks::array_chunks(hyw_iter).take(2) {
        let texts: [String; BATCH_SIZE] = batch.map(|hyw| hyw.to_string());
        let text_refs: [&str; BATCH_SIZE] = std::array::from_fn(|i| texts[i].as_str());
        let embeddings = client.embed_text(&text_refs).await?;

        for (hyw, embedding) in batch.iter().zip(embeddings.iter()) {
            writeln!(lock, "[{:>6}/{size}] {hyw} ({}, {}, {}...)", hyw.to_index(), embedding[0], embedding[1], embedding[2]).unwrap();
            // lock.flush().unwrap();
        }
    }

    Ok(())
}
