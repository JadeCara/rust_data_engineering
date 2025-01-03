use reqwest::{header, Client};
use serde_json::json;
use serde_json::Value;
use std::io;
use std::io::Write;

pub async fn run_chat_loop(
    client: &Client,
    api_key: &str,
    url: &str,
) -> Result<(), reqwest::Error> {
    let mut conversation: String = String::from("The following is a conversation with an AI Assistant:");
    loop {
        print!("Human: ");
        io::stdout().flush().unwrap();

        let user_input: String = read_user_input();

        if user_input.to_lowercase() == "exit" || user_input.to_lowercase() == "quit" {
            break;
        }

        conversation.push_str("Human: ");
        conversation.push_str(&user_input);
        conversation.push_str("AI: ");

        let json: Value = json!({
            "model": "gpt-3.5-turbo-instruct",
            "prompt": conversation,
            "max_tokens": 150,
            "temperature": 0.5,
            "top_p": 1.0,
            "frequency_penalty": 0.0,
            "presence_penalty": 0.0,
            "stop": ["Human:", "AI:"]
        });

        let body: Value = call_api(client, api_key, url, json).await?;
        print!("{}", body);
        let ai_response: &str = get_ai_response(&body);

        print!("AI: {}", ai_response);

        conversation.push_str(ai_response);
        conversation.push_str("\n");
    }
    Ok(())
}

pub async fn call_api(
    client: &Client,
    api_key: &str,
    url: &str,
    json: Value,
) -> Result<Value, reqwest::Error> {
    let response: reqwest::Response = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&json)
        .send()
        .await?;

    let body: Value = response.json().await?;
    Ok(body)
}

pub fn get_ai_response(body: &Value) -> &str {
    body["choices"][0]["text"].as_str().unwrap().trim()
}

pub fn read_user_input() -> String {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}