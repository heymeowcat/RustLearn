use reqwest;
use tokio;
use std::error::Error;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];
    let fetches = urls.into_iter().map(|url| fetch_data(url.to_string()));
    let results: Vec<_> = join_all(fetches).await;
    for result in results {
        match result {
            Ok(data) => println!("Fetched data: {}", data),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}

async fn fetch_data(url: String) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(&url).await?;
    let text = response.text().await?;
    Ok(text)
}