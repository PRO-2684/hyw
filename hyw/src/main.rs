#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use argh::FromArgs;
use hyw::{ApiClient, Embedding, Hyw, search};
use instant_distance::HnswMap;
use postcard::from_io;
use std::{fmt, fs::File, io::Write, path::Path, str::FromStr};

/// 合义维
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))]
struct Args {
    /// the action to take, can be 'list'/'l', 'query'/'q' or 'search'/'s' (default)
    #[argh(positional, default = "Default::default()")]
    action: Action,
    /// key for SiliconFlow API, required for search, using environment variable SILICONFLOW_API_KEY if not provided
    #[argh(option, short = 'k')]
    api_key: Option<String>,
    /// path to the embedding map file, default is "./hyw.postcard"
    #[argh(option, short = 'm', default = "\"./hyw.postcard\".to_string()")]
    map_path: String,
}

/// Possible actions.
enum Action {
    /// List all hyw.
    List,
    /// Query hyw from index and vice versa.
    Query,
    /// Search hyw.
    Search,
}

impl FromStr for Action {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "list" | "l" => Ok(Action::List),
            "query" | "q" => Ok(Action::Query),
            "search" | "s" => Ok(Action::Search),
            _ => Err("invalid action, must be 'list'/'l', 'query`/'q' or 'search'/'s'"),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List => write!(f, "list"),
            Self::Query => write!(f, "query"),
            Self::Search => write!(f, "search"),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::Search
    }
}

#[compio::main]
async fn main() {
    let args: Args = argh::from_env();
    let Args {
        action,
        api_key,
        map_path,
    } = args;

    match action {
        Action::List => list(),
        Action::Query => query(),
        Action::Search => search_hyw(api_key, map_path).await,
    }

    println!("Exiting. Goodbye!");
}

/// List all hyw.
fn list() {
    for hyw in Hyw::all() {
        println!("{hyw}");
    }
}

/// Query hyw from index and vice versa.
fn query() {
    let mut query = "1".to_string(); // Prevent empty on first loop
    let stdin = std::io::stdin();
    while !query.trim().is_empty() {
        query.clear();
        eprint!("\nEnter query (or press Enter to exit): ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut query).unwrap();
        let query = query.trim();
        if query.is_empty() {
            break;
        }
        if let Ok(index) = query.parse() {
            match Hyw::from_index(index) {
                Some(hyw) => println!("Hyw for index {index}: {hyw}"),
                None => eprintln!("No hyw found for index {index}"),
            }
        } else {
            match Hyw::from_str(query) {
                Ok(hyw) => println!("Index for hyw {hyw}: {}", hyw.to_index()),
                Err(_) => eprintln!("Invalid hyw string: {query}"),
            }
        }
    }
}

/// Search hyw.
async fn search_hyw(api_key: Option<String>, map_path: String) {
    // Initialize API client
    let api_key = api_key.unwrap_or_else(|| {
        std::env::var("SILICONFLOW_API_KEY")
            .expect("SILICONFLOW_API_KEY environment variable not set")
    });
    let client = ApiClient::new(&api_key).expect("Failed to create API client");

    // Deserialize HNSW map
    eprintln!("Loading embedding map from {map_path}");
    let map_path = Path::new(&map_path);
    let file = File::open(&map_path).expect("Failed to open embedding map file");
    let mut buffer = vec![0u8; 8192]; // Buffer for deserialization
    let map: HnswMap<Embedding, usize> = from_io((file, &mut buffer))
        .expect("Failed to deserialize embedding map")
        .0;

    eprintln!("Embedding map has been loaded!");

    let mut search_term = "1".to_string(); // Prevent empty on first loop
    let stdin = std::io::stdin();
    while !search_term.trim().is_empty() {
        search_term.clear();
        eprint!("\nEnter search term (or press Enter to exit): ");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut search_term).unwrap();
        let query = search_term.trim();
        if query.is_empty() {
            break;
        }
        if let Err(e) = search(&map, &client, query).await {
            eprintln!("Error during search: {e}");
        }
    }
}
