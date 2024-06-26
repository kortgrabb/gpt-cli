use serde::{Serialize, Deserialize};

mod gpt;
mod ui;

#[derive(Serialize, Deserialize)]
struct Config {
    api_key: String,
    engine: String,
    max_tokens: usize,
    temperature: f32,
    top_p: f32,
}

fn read_config() -> Config {
    // check if the config file exists
    if !std::path::Path::new("config.json").exists() {
        // if it doesn't, create it
        let config = Config {
            api_key: "".to_string(),
            engine: "gpt-4o".to_string(),
            max_tokens: 150,
            temperature: 0.7,
            top_p: 1.0,
        };

        let config_json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write("config.json", config_json).unwrap();
    }

    let config_json = std::fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(&config_json).unwrap();

    config
}

fn main() {    
    let config = read_config();

    if config.api_key.is_empty() {
        eprintln!("Error: API key is missing. Please add your API key to config.json");
        return;
    }

    println!("+-----------Welcome to ChatGPT CLI-----------+");
    println!("| Type ';exit' to exit the program           |");
    println!("| Edit config.json to change settings        |");
    println!("+--------------------------------------------+");

    let mut chatgpt_client = gpt::GPTClient::new(&config);
    let t_runtime = tokio::runtime::Runtime::new().unwrap();

    loop {
        match ui::get_user_input() {
            Some(input) => {
                if input.to_lowercase() == ";exit" {
                    break;
                }

                let response = t_runtime.block_on(chatgpt_client.get_response(&input));
                match response {
                    Ok(result) => ui::display_response(&result),
                    Err(err) => eprintln!("Error: {}", err),
                }

                //ui::print_border();
            }
            None => break,
        }
    }
}