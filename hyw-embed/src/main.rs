#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Fucking windows.")]

use hyw_base::Hyw;
use hyw_embed::{ApiClient, EmbedError};
use instant_distance::{Builder, Search};
use itermore::IterArrayChunks;
use postcard::{from_io, to_io};
use std::io::Write;

// const BATCH_SIZE: usize = 128;
const BATCH_SIZE: usize = 16; // DEBUG

#[compio::main]
async fn main() -> Result<(), EmbedError> {
    // Parse arguments (TODO: Use proper argument parser)
    let mut args = std::env::args().skip(1);
    let api_key = args.next().expect("Please provide SILICON_FLOW_API_KEY as the first argument");
    let data_path = args.next().unwrap_or_else(|| "hyw_embeddings.hnsw".to_string());

    // Initialize API client & lock stdout
    let client = ApiClient::new(&api_key).expect("Failed to create API client");
    let mut lock = std::io::stdout().lock();

    // Prepare Hyw iterator and batches
    let hyw_iter = Hyw::all();
    let size = hyw_iter.size_hint().0;
    let hyw_batches = hyw_iter.arrays::<BATCH_SIZE>();

    // Data to construct instant_distance::HnswMap
    let mut points = Vec::with_capacity(size); // Embeddings
    let mut values = Vec::with_capacity(size); // Indices

    // Process batches
    for batch in hyw_batches.take(2) { // DEBUG: Process only first 2 batches
        let texts: [String; BATCH_SIZE] = batch.map(|hyw| hyw.to_string());
        let text_refs: [&str; BATCH_SIZE] = std::array::from_fn(|i| texts[i].as_str());
        let embeddings = client.embed_text(&text_refs).await?;

        for (hyw, embedding) in batch.iter().zip(embeddings.iter()) {
            writeln!(lock, "[{:>7}/{size}] {hyw} ({}, {}, {}...)", hyw.to_index(), embedding[0], embedding[1], embedding[2]).unwrap();
        }

        points.extend(embeddings);
        values.extend(batch.map(|hyw| hyw.to_index()));
    }

    // TODO: Process remainders

    // DEBUG: Test query
    let test_query = points[0].clone();
    let test_hyw = Hyw::from_index(values[0]).expect("Cannot reconstruct Hyw from index");
    writeln!(lock, "Test query Hyw: {test_hyw}").unwrap();

    // Construct HnswMap
    // PointId (result.pid) is not guaranteed to be in sync with Hyw index, so we do not use Hnsw, but store Hyw indices in HnswMap as the value.
    let map = Builder::default().build(points, values);

    // DEBUG: Test search
    let mut search = Search::default();
    let results = map.search(&test_query, &mut search);
    for (i, result) in results.take(5).enumerate() {
        let hyw = Hyw::from_index(*result.value).expect("Cannot reconstruct Hyw from index");
        writeln!(lock, "Result #{i}: {hyw} (Distance: {}, id: {})", result.distance, result.pid.into_inner()).unwrap();
    }

    // TODO: Serialize HnswMap

    Ok(())
}
