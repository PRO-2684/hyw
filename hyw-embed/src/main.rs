#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Fucking windows.")]

use compio::runtime::time::sleep;
use hyw_base::Hyw;
use hyw_embed::{ApiClient, EmbedError, Embedding};
use itermore::IterArrayChunks;
use postcard::{from_io, to_io};
use std::{fs::File, io::{Write, Seek}, path::Path, time::Duration};

const BATCH_SIZE: usize = 32;
const RPM: u32 = 2000; // Rate limit: 2k requests per minute
const DELAY: Duration = Duration::from_millis(60_000 / RPM as u64);

#[compio::main]
async fn main() -> Result<(), EmbedError> {
    // Parse arguments (TODO: Use proper argument parser)
    let mut args = std::env::args().skip(1);
    let api_key = args
        .next()
        .expect("Please provide SILICON_FLOW_API_KEY as the first argument");
    let data_path = args
        .next()
        .unwrap_or_else(|| "./hyw_embeddings.postcard".to_string());
    let data_path = Path::new(&data_path);

    // Initialize API client
    let client = ApiClient::new(&api_key).expect("Failed to create API client");
    let mut stderr = std::io::stderr().lock();

    // Prepare Hyw iterator
    let size = Hyw::all().size_hint().0;
    let total_batches = (size + BATCH_SIZE - 1) / BATCH_SIZE;
    let batch_width = total_batches.to_string().len();

    // Load existing data or start fresh
    let mut data: Vec<Embedding> = if data_path.exists() {
        let file = File::open(&data_path)?;
        let mut buffer = vec![0u8; 8192]; // Buffer for deserialization
        let (data, _) =
            from_io((file, &mut buffer)).map_err(|e| EmbedError::DataDeserialization(e))?;
        data
    } else {
        Vec::with_capacity(size)
    };

    let current_count = data.len();
    if current_count >= size {
        writeln!(
            stderr,
            "All embeddings already generated ({current_count}/{size}). Exiting."
        )
        .unwrap();
        return Ok(());
    }

    writeln!(stderr, "Resuming from {current_count}/{size} embeddings...").unwrap();

    // Skip already processed items
    let hyw_iter = Hyw::all().skip(current_count);
    let mut hyw_batches = hyw_iter.arrays::<BATCH_SIZE>();

    // Process full batches
    let mut file = File::create(&data_path)?;
    let mut batch_num = (current_count / BATCH_SIZE) + 1;
    while let Some(batch) = hyw_batches.next() {
        let texts: [String; BATCH_SIZE] = batch.map(|hyw| hyw.to_string());
        let text_refs: [&str; BATCH_SIZE] = std::array::from_fn(|i| texts[i].as_str());
        let embeddings = client.embed_text(&text_refs).await?;

        for embedding in embeddings.iter() {
            data.push(embedding.clone());
        }

        // Save after each batch
        file.rewind()?;
        to_io(&data, &mut file).map_err(|e| EmbedError::DataSerialization(e))?;
        file.flush()?;

        write!(stderr, "\r[{batch_num:>batch_width$}/{total_batches}] Processed batch, saved {}/{size} embeddings", data.len()).unwrap();
        stderr.flush().unwrap();
        batch_num += 1;

        // Respect rate limit
        sleep(DELAY).await;
    }

    // Process remainders
    let remainder: Vec<Hyw> = hyw_batches.into_remainder().collect();
    if !remainder.is_empty() {
        let texts: Vec<String> = remainder.iter().map(|hyw| hyw.to_string()).collect();
        let text_refs: Vec<&str> = texts.iter().map(String::as_str).collect();
        let embeddings = client.embed_text(&text_refs).await?;

        for embedding in embeddings.iter() {
            data.push(embedding.clone());
        }

        // Save final data
        file.rewind()?;
        to_io(&data, &mut file).map_err(|e| EmbedError::DataSerialization(e))?;
        file.flush()?;

        write!(stderr, "\r[{batch_num:>batch_width$}/{total_batches}] Processed remainder, saved {}/{size} embeddings", data.len()).unwrap();
    }

    writeln!(
        stderr,
        "\nCompleted! All {size} embeddings generated and saved."
    )
    .unwrap();

    Ok(())
}
