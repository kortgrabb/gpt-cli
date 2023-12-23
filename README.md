# Rust GPT Console Application

This application is a Rust-based console interface for interacting with OpenAI's GPT. It features real-time streaming of responses, allowing users to see the GPT responses as they are generated.

## Features

- **GPT Integration**: Communicate with OpenAI's GPT model in real-time.
- **Configurable**: Easy to configure with your OpenAI API key and other settings.

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

---