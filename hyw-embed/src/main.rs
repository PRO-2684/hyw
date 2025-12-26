#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Fucking windows.")]

use compio::{runtime::time::interval, signal::ctrl_c};
use futures_util::{FutureExt, select};
use hyw_base::Hyw;
use hyw_embed::{ApiClient, EmbedError, Embedding};
use instant_distance::{Builder, HnswMap, Search};
use itermore::IterArrayChunks;
use postcard::{from_io, to_io, Error as PostcardError};
use std::{fs::File, io::{Error as IoError, Write}, path::Path, pin::pin, time::Duration};
use thiserror::Error;

const BATCH_SIZE: usize = 32;
const RPM: u64 = 2000; // Rate limit: 2k requests per minute
const DELAY: Duration = Duration::from_millis(60_000 / RPM);

#[derive(Error, Debug)]
enum MainError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("Postcard serialization/deserialization error: {0}")]
    Postcard(#[from] PostcardError),
    #[error("Embedding error: {0}")]
    Embed(#[from] EmbedError),
}

#[compio::main]
async fn main() -> Result<(), MainError> {
    // Parse arguments (TODO: Use proper argument parser)
    let mut args = std::env::args().skip(1);
    let api_key = args
        .next()
        .expect("Please provide SILICON_FLOW_API_KEY as the first argument");
    let map_path = args
        .next()
        .unwrap_or_else(|| "./hyw.postcard".to_string());
    // Serialized mapping
    let map_path = Path::new(&map_path);
    // Temporary store of embeddings during processing
    let tmp_path = map_path.with_extension("postcard.tmp");

    // Initialize API client
    let client = ApiClient::new(&api_key).expect("Failed to create API client");

    let map = if map_path.exists() {
        // Deserialize existing map
        eprintln!("Loading existing embedding map from {}", map_path.display());
        let file = File::open(&map_path)?;
        let mut buffer = vec![0u8; 8192]; // Buffer for deserialization
        from_io((file, &mut buffer))?.0
    } else {
        // Create new map
        eprintln!("Creating new embedding map at {}", map_path.display());
        // Load existing data or start fresh
        let mut data: Vec<Embedding> = if tmp_path.exists() {
            eprintln!("Resuming from temporary file {}", tmp_path.display());
            let file = File::open(&tmp_path)?;
            let mut buffer = vec![0u8; 8192]; // Buffer for deserialization
            from_io((file, &mut buffer))?.0
        } else {
            eprintln!("No temporary file found, starting fresh.");
            Vec::new()
        };

        let embed_result = embed_all(&client, &mut data).await;
        match &embed_result {
            Ok(()) => eprintln!("Embedding process completed successfully. Saving temp data..."),
            Err(e) => eprintln!("Embedding process failed: {e}. Saving progress for later resumption..."),
        };

        // Save tmp data
        let file = File::create(&tmp_path)?;
        to_io(&data, file)?;

        // Create map
        eprintln!("Building HNSW map...");
        let indices: Vec<_> = (0..data.len()).collect();
        let map = Builder::default().build(data, indices);

        // Save final map
        eprintln!("Saving final embedding map to {}...", map_path.display());
        let file = File::create(&map_path)?;
        to_io(&map, file)?;

        map
    };

    eprintln!("Embedding map is ready! Starting search...");

    let mut query = "1".to_string(); // Prevent empty on first loop
    let stdin = std::io::stdin();
    while !query.trim().is_empty() {
        query.clear();
        eprint!("\nEnter search query (or press Enter to exit): ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut query).unwrap();
        let query = query.trim();
        if query.is_empty() {
            break;
        }
        search(&map, &client, query).await?;
    }

    println!("Exiting. Goodbye!");

    Ok(())
}

/// Helper function to embed text with Ctrl-C handling.
async fn embed_with_interrupt(client: &ApiClient, text_refs: &[&str]) -> Result<Vec<Embedding>, EmbedError> {
    let ctrlc = pin!(ctrl_c());
    select! {
        res = ctrlc.fuse() => {
            res.map_err(|e| EmbedError::UnknownApiError {
                code: http::StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Ctrl-C handler error: {e}")
            })?;
            let mut stderr = std::io::stderr().lock();
            writeln!(stderr, "\nReceived Ctrl-C, please wait while we're saving progress...").unwrap();
            Err(EmbedError::UnknownApiError {
                code: http::StatusCode::INTERNAL_SERVER_ERROR,
                message: "Interrupted by user".to_string()
            })
        }
        result = client.embed_text(text_refs).fuse() => {
            result
        }
    }
}

/// Embeds all Hyw items in batches, updating the provided data vector.
async fn embed_all(client: &ApiClient, data: &mut Vec<Embedding>) -> Result<(), EmbedError> {
    let mut stderr = std::io::stderr().lock();
    // Prepare Hyw iterator
    let size = Hyw::all().size_hint().0;
    let total_batches = (size + BATCH_SIZE - 1) / BATCH_SIZE;
    let batch_width = total_batches.to_string().len();
    data.reserve(size - data.len());

    let current_count = data.len();
    if current_count >= size {
        writeln!(
            stderr,
            "All embeddings already generated ({current_count}/{size}). Skipping embedding process."
        )
        .unwrap();
        return Ok(());
    }

    writeln!(stderr, "Starting from {current_count}/{size} embeddings...").unwrap();

    // Skip already processed items
    let hyw_iter = Hyw::all().skip(current_count);
    let mut hyw_batches = hyw_iter.arrays::<BATCH_SIZE>();

    // Process full batches
    let mut interval = interval(DELAY);
    let mut batch_num = (current_count / BATCH_SIZE) + 1;
    while let Some(batch) = hyw_batches.next() {
        let texts: [String; BATCH_SIZE] = batch.map(|hyw| hyw.to_string());
        let text_refs: [&str; BATCH_SIZE] = std::array::from_fn(|i| texts[i].as_str());

        let embeddings = embed_with_interrupt(&client, &text_refs).await?;
        data.extend(embeddings);

        write!(stderr, "\r[{batch_num:>batch_width$}/{total_batches}] Processed batch, {}/{size} embeddings", data.len()).unwrap();
        stderr.flush().unwrap();
        batch_num += 1;

        // Respect rate limit
        interval.tick().await;
    }

    // Process remainders
    let remainder = hyw_batches.into_remainder();
    let texts: Vec<String> = remainder.map(|hyw| hyw.to_string()).collect();
    if !texts.is_empty() {
        let text_refs: Vec<&str> = texts.iter().map(String::as_str).collect();
        let embeddings = embed_with_interrupt(&client, &text_refs).await?;
        data.extend(embeddings);

        write!(stderr, "\r[{batch_num:>batch_width$}/{total_batches}] Processed remainder, {}/{size} embeddings", data.len()).unwrap();
    }

    writeln!(
        stderr,
        "\nCompleted! All {size} embeddings generated and saved."
    )
    .unwrap();

    Ok(())
}

/// Searches for the given query.
async fn search(map: &HnswMap<Embedding, usize>, client: &ApiClient, query: &str) -> Result<(), EmbedError> {
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
