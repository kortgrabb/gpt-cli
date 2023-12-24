use serde::{Serialize, Deserialize};

mod gpt;
mod ui;

#[derive(Serialize, Deserialize)]
struct Config {
    api_key: String,
    engine: String,
}

fn read_config() -> Config {
    // check if the config file exists
    if !std::path::Path::new("config.json").exists() {
        // if it doesn't, create it
        let config = Config {
            api_key: "".to_string(),
            engine: "gpt-3.5-turbo".to_string(),
        };

        let config_json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write("config.json", config_json).unwrap();
    }

    // read the config file
    let config_json = std::fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(&config_json).unwrap();

    config
}

fn main() {    
    
    let config = read_config();

    if config.api_key.is_empty() {
        eprintln!("Error: API key is empty. Please add your API key to config.json");
        return;
    }

    let mut chatgpt_client = gpt::GPTClient::new(&config);
    let t_runtime = tokio::runtime::Runtime::new().unwrap();

    loop {
        match ui::get_user_input() {
            Some(input) => {
                if input.to_lowercase() == "exit" {
                    break;
                }
                
                // Use the runtime to block on the future
                match t_runtime.block_on(chatgpt_client.get_response(&input)) {
                    Ok(response) => ui::display_response(&response),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            None => break,
        }
    }
}