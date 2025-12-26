//! # `hyw` library crate
//!
//! If you are reading this, you are reading the documentation for the `hyw` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

// Re-export public APIs from hyw-base and hyw-embed
pub use hyw_base::{Hyw, HywIterator, HE, YI, WEI};
pub use hyw_embed::{ApiClient, EmbedError, Embedding};

use instant_distance::{HnswMap, Search};

/// Searches for the given query, printing the top 5 results.
pub async fn search(map: &HnswMap<Embedding, usize>, client: &ApiClient, query: &str) -> Result<(), EmbedError> {
    let query_embedding = &client.embed_text(&[query]).await?[0];
    let mut search_state = Search::default(); // only used by the library internally
    let results = map.search(&query_embedding, &mut search_state);

    println!("Top 5 results for query: \"{query}\"");
    for (rank, result) in results.take(5).enumerate() {
        let id = result.value;
        let distance = result.distance;
        let hyw = Hyw::from_index(*id).unwrap();
        println!("#{}: {hyw} (Distance: {distance:.4})", rank + 1);
    }

    Ok(())
}
