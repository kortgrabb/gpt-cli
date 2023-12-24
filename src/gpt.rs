use reqwest::{Client, Error};
use serde_json::{json, Value};

pub struct GPTClient {
    client: Client,
    engine: String,
    api_key: String,
    message_history: Vec<Value>,
}

impl GPTClient {
    pub fn new(config: &super::Config) -> GPTClient {
        GPTClient {
            client: Client::new(),
            api_key: config.api_key.to_string(),
            message_history: vec![json!({
                "role": "system",
                "content": "You are gpt integrated into the console of the user. Because of your limitations, you can only respond to the user's messages in plain text. You will not provide formated responses in Markdown."
            })],

            engine: config.engine.to_string(),
        }
    }

    pub async fn get_response(&mut self, user_input: &str) -> Result<String, Error> {
        // Add user message to history
        self.message_history.push(json!({
            "role": "user",
            "content": user_input
        }));

        // Prepare request body with message history
        let body = json!({
            "model": self.engine,
            "messages": self.message_history
        });

        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        // Extract and add AI's response to history
        let ai_response = response["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
        
       if ai_response.is_empty() {
            eprintln!("Error: Invalid model name in config.json");
       }

        self.message_history.push(json!({
            "role": "assistant",
            "content": &ai_response
        }));

        if self.message_history.len() > 10 {
            self.message_history.remove(0);
        }

        Ok(ai_response)
    }
}
