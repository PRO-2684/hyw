#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use argh::FromArgs;
use hyw::{ApiClient, Embedding, search};
use instant_distance::HnswMap;
use postcard::from_io;
use std::{fs::File, io::Write, path::Path};

/// Querying embeddings for hyw.
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))]
struct Args {
    /// key for SiliconFlow API
    #[argh(option, short = 'k')]
    api_key: String,
    /// path to the embedding map file
    #[argh(option, short = 'm', default = "\"./hyw.postcard\".to_string()")]
    map_path: String,
}

#[compio::main]
async fn main() {
    // Parse arguments (TODO: Use proper argument parser)
    let args: Args = argh::from_env();
    let Args { api_key, map_path } = args;

    // Initialize API client
    let client = ApiClient::new(&api_key).expect("Failed to create API client");

    // Deserialize HNSW map
    eprintln!("Loading embedding map from {}", map_path);
    let map_path = Path::new(&map_path);
    let file = File::open(&map_path).expect("Failed to open embedding map file");
    let mut buffer = vec![0u8; 8192]; // Buffer for deserialization
    let map: HnswMap<Embedding, usize> =
        from_io((file, &mut buffer)).expect("Failed to deserialize embedding map").0;

    eprintln!("Embedding map has been loaded!");

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
        if let Err(e) = search(&map, &client, query).await {
            eprintln!("Error during search: {e}");
        }
    }

    println!("Exiting. Goodbye!");
}
