mod gpt;
mod ui;

struct Config {
    api_key: String,
    engine: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let api_key = args[1].clone();

        let engine = if args.len() > 2 {
            args[2].clone()
        } else {
            String::from("gpt-3.5-turbo")
        };

        Ok(Config { api_key, engine })
    } 
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chatgpt <api_key> [engine]");
        std::process::exit(1);
    }
    
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let api_key = config.api_key;
    let engine = config.engine;

    // Create a new GPT client
    let mut chatgpt_client = gpt::GPTClient::new(&api_key, &engine);

    // Create a new runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    loop {
        match ui::get_user_input() {
            Some(input) => {
                if input.to_lowercase() == "exit" {
                    break;
                }
                
                // Use the runtime to block on the future
                match rt.block_on(chatgpt_client.get_response(&input)) {
                    Ok(response) => ui::display_response(&response),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            None => break,
        }
    }
}