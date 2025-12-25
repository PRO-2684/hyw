#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Fucking windows.")]

use compio::{runtime::time::interval, signal::ctrl_c};
use futures_util::{FutureExt, select};
use hyw_base::Hyw;
use hyw_embed::{ApiClient, EmbedError, Embedding};
use itermore::IterArrayChunks;
use postcard::{from_io, to_io};
use std::{fs::File, io::{Error as IoError, ErrorKind, Write}, path::Path, pin::pin, time::Duration};

const BATCH_SIZE: usize = 32;
const RPM: u64 = 2000; // Rate limit: 2k requests per minute
const DELAY: Duration = Duration::from_millis(60_000 / RPM);

#[compio::main]
async fn main() -> Result<(), IoError> {
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

    // Load existing data or start fresh
    let mut data: Vec<Embedding> = if data_path.exists() {
        let file = File::open(&data_path)?;
        let mut buffer = vec![0u8; 8192]; // Buffer for deserialization
        let (data, _) =
            from_io((file, &mut buffer)).map_err(|e| IoError::new(ErrorKind::Other, e))?;
        data
    } else {
        Vec::new()
    };

    match run(client, &mut data).await {
        Ok(()) => eprintln!("Embedding process completed successfully."),
        Err(e) => eprintln!("Embedding process failed: {e}"),
    };

    // Save data
    let file = File::create(&data_path)?;
    to_io(&data, file).map_err(|e| IoError::new(ErrorKind::Other, e))?;

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

/// Run the embedding process.
async fn run(client: ApiClient, data: &mut Vec<Embedding>) -> Result<(), EmbedError> {
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
