mod config;
mod error;
mod parser;

use clap::Parser;
use std::fs;

use crate::config::{build_config, ChatSource, CliArgs};
use crate::error::Result;
use crate::parser::{
    parse_chatgpt_to_markdown, parse_claude_to_markdown, parse_deepseek_to_markdown,
};

fn main() -> Result<()> {
    // Setup app metadata
    let app_name = env!("APP_NAME");
    let app_version = env!("APP_VERSION");
    let app_build = env!("APP_BUILD");

    // Parse command line arguments
    let cli_args = CliArgs::parse();

    // App banner
    println!("{} v{} (build {})", app_name, app_version, app_build);

    // Get configuration
    let markdown_config = build_config(&cli_args)?;

    // Create output directory
    if let Some(parent) = markdown_config.output_file.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Input: {}", markdown_config.input_file.display());
    println!("Output: {}", markdown_config.output_file.display());

    // Read input file
    let json_content = fs::read_to_string(&markdown_config.input_file)?;

    match markdown_config.chat_source {
        ChatSource::Claude => {
            // Parse
            let chat = serde_json::from_str(&json_content)?;

            // Convert to markdown and write to file
            let markdown = parse_claude_to_markdown(&chat, &markdown_config)?;
            fs::write(&markdown_config.output_file, markdown)?;
        }
        ChatSource::ChatGPT => {
            // Parse
            let chat = serde_json::from_str(&json_content)?;

            // Convert to markdown and write to file
            let markdown = parse_chatgpt_to_markdown(&chat, &markdown_config)?;
            fs::write(&markdown_config.output_file, markdown)?;
        }
        ChatSource::DeepSeek => {
            // Parse
            let chat = serde_json::from_str(&json_content)?;

            // Convert to markdown and write to file
            let markdown = parse_deepseek_to_markdown(&chat, &markdown_config)?;
            fs::write(&markdown_config.output_file, markdown)?;
        }
    }

    println!("Done.");

    Ok(())
}
