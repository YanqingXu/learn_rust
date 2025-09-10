use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    body: String,
    userId: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    println!("Fetching URL: {}", url);

    let post = reqwest::get(url)
        .await?
        .json::<Post>()
        .await?;

    println!("\nFetched Post:\n{:#?}", post);

    Ok(())
}