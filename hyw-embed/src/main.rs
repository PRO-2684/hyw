#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use hyw_embed::ApiClient;

#[compio::main]
async fn main() {
    let api_key = std::env::var("SILICON_FLOW_API_KEY").expect("SILICON_FLOW_API_KEY not set");
    let client = ApiClient::new(&api_key).await;
    let texts = vec!["你好，世界！", "测试文本"];
    let embeddings = client.embed_text(&texts).await;
    for (i, embedding) in embeddings.iter().enumerate() {
        println!("Embedding for text {}: {:?}", i, &embedding[..5]); // Print first 5 values
    }
}
