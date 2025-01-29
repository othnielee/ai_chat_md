# AI Chat Markdown

This Rust utility parses AI chat messages in JSON format from Claude, ChatGPT or DeepSeek, and saves them as Markdown files. It provides a convenient way to convert chat logs into a readable and shareable format.

## Installation

1. Make sure you have Rust installed on your system. You can download and install Rust from the official website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

2. Clone this repository:

3. Change to the project directory:

   ```
   cd ai_chat_md
   ```

4. Build the project:

   ```
   cargo build --release
   ```

   The compiled binary will be available in the `target/release` directory.

## Basic Functionality

The AI Chat Markdown utility performs the following tasks:

1. Reads AI chat messages in JSON format from an input file.
2. Parses the chat messages based on the specified chat source (Claude, ChatGPT or DeepSeek).
3. Converts the parsed chat messages into Markdown format.
4. Writes the Markdown output to a specified output file.

## Command Line Usage

To use the AI Chat Markdown utility, run the compiled binary with the following command line arguments:

```
ai_chat_md [OPTIONS]
```

Available options are below. With the exception of the input file option, if an option is not provided, the default value from the configuration file or environment variable will be used.

- `-s, --chat-source <CHAT_SOURCE>`: Specifies the chat source. Valid values are `claude`, `chatgpt` and `deepseek`.
- `-t, --timezone <TIMEZONE>`: Sets the timezone for the Markdown output.
- `-d, --base-dir <BASE_DIR>`: Specifies the base directory for input and output files.
- `-p, --inline-output`: Forces saving the output to the same directory as the input file.
- `-i, --input-file <INPUT_FILE>`: Specifies the input chat file. This option is required. The input file must be stored under the designated `data` directory and should contain the chat messages in JSON format, downloaded via the browser developer tools.
- `-o, --output-file <OUTPUT_FILE>`: Specifies the output Markdown file.
- `-u, --user-name <USER_NAME>`: Sets the name for the user.
- `-a, --ai-name <AI_NAME>`: Sets the name for the AI assistant.
- `-r, --reasoning`: Shows the reasoning for each message.
- `-c, --config <CONFIG>`: Specifies the path to the configuration file.
- `-h, --help`: Prints help information.
- `-V, --version`: Prints version information.

## Configuration

The utility can be configured using a configuration file, environment variables, and command line arguments. The configuration file must be provided in JSON format.

The available configuration options are:

- `chat_source`: The chat source. Valid values are `claude`, `chatgpt` and `deepseek`.
- `timezone`: The timezone for the Markdown output.
- `base_dir`: The base directory for input and output files.
- `inline_output`: Forces saving the output to the same directory as the input file.
- `user_name`: The name for the user.
- `ai_name`: The name for the AI assistant.
- `reasoning`: Shows the reasoning (chain of thought) for each message if available.

Environment variables can also be used to override the configuration values. The environment variable names are prefixed with `CHAT_` and use underscores as separators. For example, `CHAT_CHAT_SOURCE` corresponds to the `chat_source` configuration option.

Command line arguments take precedence over environment variables and configuration file values.

## License

This project is licensed under the MIT License.
