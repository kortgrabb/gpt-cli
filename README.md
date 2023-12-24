# Rust GPT Console Application

This application is a Rust-based console interface for interacting with OpenAI's GPT.

## Features

- **GPT Integration**: Chat with any model you want in just your terminal!
- **Configurable**: Easily adjust and tune the model using the following configuration values:
   - `api-key`: The API key needed for authentication and accessing the OpenAI GPT model.
   - `model`: Specify the GPT model to be used for generating responses.
   - `max-tokens`: Set the limit for the number of tokens in the generated response.
   - `temperature`: Influence the randomness of the generated response. Higher values increase randomness, while lower values make the output more deterministic.
   - `top_p`: Control the diversity of the generated response. Set the cumulative probability threshold for the selection of the next token. Higher values increase diversity, while lower values make the output more deterministic.

Let me know if you need any further assistance!

## Requirements

- Rust Programming Language
- Cargo (Rust's Package Manager)
- OpenAI API Key

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/kortgrabb/gpt-cli.git
   cd rust-GPT-console-app
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

## Usage

1. Run the application:
   ```bash
   cargo run
   ```

2. Input your queries and see GPT's responses in real-time.

3. Type 'exit' to quit the application.

## Modules

- **main.rs**: Entry point of the application.
- **gpt.rs**: Handles the interaction with the OpenAI API, including streaming responses.
- **ui.rs**: Manages the user interface, including input prompts and displaying responses.

## Contributing

Contributions to this project are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature.
3. Commit your changes.
4. Push to your branch.
5. Open a pull request.

## License

This project is licensed under [MIT] - see the LICENSE file for details.