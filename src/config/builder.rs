use super::cli::CliArgs;
use super::constants::*;
use super::error::{ConfigError, Result};
use super::model::{AppConfig, MarkdownConfig};
use super::paths::resolve_paths;
use config::{Config, Environment, File};
use dotenv::dotenv;
use std::path::Path;

fn build_app_config(cli_args: &CliArgs) -> Result<AppConfig> {
    let mut builder = Config::builder();

    // Add default values
    builder = builder.set_default(KEY_CHAT_SOURCE, DEFAULT_CHAT_SOURCE)?;
    builder = builder.set_default(KEY_TIMEZONE, DEFAULT_TIMEZONE)?;
    builder = builder.set_default(KEY_BASE_DIR, DEFAULT_BASE_DIR)?;
    builder = builder.set_default(KEY_INLINE_OUTPUT, DEFAULT_INLINE_OUTPUT)?;
    builder = builder.set_default(KEY_USER_NAME, DEFAULT_USER_NAME)?;
    builder = builder.set_default(KEY_AI_NAME, DEFAULT_AI_NAME)?;

    // Add config file if it exists
    if Path::new(&cli_args.config).exists() {
        builder = builder.add_source(File::from(cli_args.config.as_ref()));
    }

    // Load .env file and environment variables
    dotenv().ok();

    builder = builder.add_source(
        Environment::with_prefix(ENV_PREFIX)
            .separator(ENV_SEPARATOR)
            .try_parsing(true),
    );

    // Build initial config
    let config = builder.build()?;

    // Create a new builder from the existing config and override
    // with in-memory environment variables if they exist
    let mut builder = Config::builder().add_source(config);

    // Handle environment variable overrides
    for (env_var, config_key) in [
        (ENV_CHAT_SOURCE, KEY_CHAT_SOURCE),
        (ENV_CHAT_TIMEZONE, KEY_TIMEZONE),
        (ENV_CHAT_BASE_DIR, KEY_BASE_DIR),
        (ENV_CHAT_INLINE_OUTPUT, KEY_INLINE_OUTPUT),
        (ENV_CHAT_INPUT_FILE, KEY_INPUT_FILE),
        (ENV_CHAT_OUTPUT_FILE, KEY_OUTPUT_FILE),
        (ENV_CHAT_USER_NAME, KEY_USER_NAME),
        (ENV_CHAT_AI_NAME, KEY_AI_NAME),
    ] {
        if let Ok(val) = std::env::var(env_var) {
            builder = builder.set_override(config_key, val)?;
        }
    }

    // Rebuild the config
    let config = builder.build()?;

    // Create the AppConfig structure
    let mut app_config = AppConfig {
        chat_source: config.get_string(KEY_CHAT_SOURCE)?.parse()?,
        timezone: config.get_string(KEY_TIMEZONE)?,
        base_dir: config.get_string(KEY_BASE_DIR)?,
        inline_output: config.get_bool(KEY_INLINE_OUTPUT)?,
        input_file: config
            .get_string(KEY_INPUT_FILE)
            .unwrap_or_else(|_| "".to_string()),
        output_file: config.get_string(KEY_OUTPUT_FILE).ok(),
        user_name: config.get_string(KEY_USER_NAME)?,
        ai_name: config.get_string(KEY_AI_NAME)?,
    };

    // Override with CLI args if provided
    if let Some(chat_source) = &cli_args.chat_source {
        app_config.chat_source = chat_source.clone();
    }
    if let Some(timezone) = &cli_args.timezone {
        app_config.timezone = timezone.clone();
    }
    if let Some(base_dir) = &cli_args.base_dir {
        app_config.base_dir = base_dir.clone();
    }
    if let Some(inline_output) = &cli_args.inline_output {
        app_config.inline_output = inline_output.clone();
    }
    if let Some(input_file) = &cli_args.input_file {
        app_config.input_file = input_file.clone();
    }
    if let Some(output_file) = &cli_args.output_file {
        app_config.output_file = Some(output_file.clone());
    }
    if let Some(user_name) = &cli_args.user_name {
        app_config.user_name = user_name.clone();
    }
    if let Some(ai_name) = &cli_args.ai_name {
        app_config.ai_name = ai_name.clone();
    }

    if app_config.input_file.is_empty() {
        return Err(ConfigError::MissingInputFile(
            "Input file is required but was not provided".to_string(),
        ));
    }

    Ok(app_config)
}

pub fn build_config(cli_args: &CliArgs) -> Result<MarkdownConfig> {
    // First build the basic app config
    let app_config = build_app_config(cli_args)?;

    // Then resolve paths and convert to markdown config
    let (_, input_path, output_path) = resolve_paths(
        &app_config.base_dir,
        app_config.inline_output,
        &app_config.input_file,
        app_config.output_file.as_ref().map(|s| s.as_str()),
    )?;

    Ok(app_config.into_markdown_config(input_path, output_path))
}
