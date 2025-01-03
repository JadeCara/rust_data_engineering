use chatbot::chatbot::run_chat_loop;
use reqwest::Client;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Load the .env file
    dotenv().ok();
    let client = Client::new();

    // use env variable OPENAI_API_KEY
    let api_key: String = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url: &str = "https://api.openai.com/v1/completions";

    run_chat_loop(&client, &api_key, url).await?;

    Ok(())
}
