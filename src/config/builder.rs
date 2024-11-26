use super::cli::CliArgs;
use super::constants::*;
use super::error::{ConfigError, Result};
use super::model::{AppConfig, MarkdownConfig};
use super::paths::resolve_paths;
use config::{Config, Environment, File};
use dirs::home_dir;
use dotenv::dotenv;
use std::path::{Path, PathBuf};

fn get_default_config_path() -> Result<PathBuf> {
    let home_dir = home_dir().ok_or(ConfigError::NoHomeDir)?;
    Ok(home_dir.join(DEFAULT_CONFIG_FILENAME))
}

fn build_app_config(cli_args: &CliArgs) -> Result<AppConfig> {
    let mut builder = Config::builder();

    // Set default values
    builder = builder.set_default(KEY_USER_NAME, DEFAULT_USER_NAME)?;
    builder = builder.set_default(KEY_TIMEZONE, DEFAULT_TIMEZONE)?;
    builder = builder.set_default(KEY_BASE_DIR, DEFAULT_BASE_DIR)?;
    builder = builder.set_default(KEY_INLINE_OUTPUT, DEFAULT_INLINE_OUTPUT)?;

    // Load .env file as environment variables.
    // Existing env vars are not overridden
    dotenv().ok();

    // Get config file path if specified and it exists
    let config_path = if let Some(config) = &cli_args.config {
        // Use config path from the cli argument
        if Path::new(config).exists() {
            println!("Using config file: {}", config);
            Some(PathBuf::from(config))
        } else {
            println!("Warning: Config file specified but not found: {}", config);
            None
        }
    } else if let Ok(env_config) = std::env::var(ENV_APP_CONFIG_PATH) {
        // Use config path from the environment variable
        if Path::new(&env_config).exists() {
            println!("Using config file: {}", env_config);
            Some(PathBuf::from(env_config))
        } else {
            println!(
                "Warning: Config file specified by environment variable but not found: {}",
                env_config
            );
            None
        }
    } else {
        // Fallback to default config file in home directory
        let default_path = get_default_config_path()?;
        if default_path.exists() {
            println!("Using default config file: {}", default_path.display());
            Some(default_path)
        } else {
            println!(
                "Warning: Default config file not found: {}",
                default_path.display()
            );
            None
        }
    };

    // If we have a valid config file path, add config
    // data to the builder, overriding default values
    // on matching config keys.
    if let Some(path) = config_path {
        builder = builder.add_source(File::from(path));
    }

    // Load all matching env vars in memory.
    // This allows us to handle env vars not specified
    // in the .env file, so it's a necessary inclusion.
    // At this point, they won't automatically override
    // values set via the config file or as defaults,
    // because they have a different naming pattern.
    // However, we explicitly map them to the appropriate
    // config keys below, effectively overriding any
    // pre-existing values.
    builder = builder.add_source(
        Environment::with_prefix(ENV_PREFIX)
            .separator(ENV_SEPARATOR)
            .try_parsing(true),
    );

    // Map environment variables to config keys.
    // Explicit mapping eliminates any issue that may arise
    // from the config crate's transformation of env vars,
    // e.g., prefix stripping, separator splitting, and case
    // conversion. It's verbose, but necessary.
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

    // Build the config
    let config = builder.build()?;

    // Get the chat source or 'None' if not specified via config
    // This will be set in the AppConfig structure
    let chat_source = match config.get_string(KEY_CHAT_SOURCE) {
        Ok(s) => Some(s.parse()?),
        Err(_) => None,
    };

    // Create the AppConfig structure
    let mut app_config = AppConfig {
        chat_source,
        ai_name: config.get_string(KEY_AI_NAME).ok(),
        user_name: config.get_string(KEY_USER_NAME)?,
        title: config.get_string(KEY_TITLE).ok(),
        timezone: config.get_string(KEY_TIMEZONE)?,
        base_dir: config.get_string(KEY_BASE_DIR)?,
        inline_output: config.get_bool(KEY_INLINE_OUTPUT)?,
        input_file: config.get_string(KEY_INPUT_FILE).ok(),
        output_file: config.get_string(KEY_OUTPUT_FILE).ok(),
    };

    // Override existing config values with cli args if provided
    if let Some(chat_source) = &cli_args.chat_source {
        app_config.chat_source = Some(chat_source.clone());
    }
    if let Some(ai_name) = &cli_args.ai_name {
        app_config.ai_name = Some(ai_name.clone());
    }
    if let Some(user_name) = &cli_args.user_name {
        app_config.user_name = user_name.clone();
    }
    if let Some(title) = &cli_args.title {
        app_config.title = Some(title.clone());
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
        app_config.input_file = Some(input_file.clone());
    }
    if let Some(output_file) = &cli_args.output_file {
        app_config.output_file = Some(output_file.clone());
    }

    // Throw an error if the chat source is not provided
    // in any of the configuration options
    if app_config.chat_source.is_none() {
        return Err(ConfigError::MissingChatSource(
            "Chat source is required but was not provided".to_string(),
        ));
    }

    // Throw an error if the input file is not provided
    // in any of the configuration options
    if app_config.input_file.is_none() {
        return Err(ConfigError::MissingInputFile(
            "Input file is required but was not provided".to_string(),
        ));
    }

    // Update the ai name if it was not explicitly set and the existing
    // name doesn't match the default name for the chat source
    if let Some(chat_source) = &app_config.chat_source {
        let source_from_cli = cli_args.chat_source.is_some();
        let source_from_env = std::env::var(ENV_CHAT_SOURCE).is_ok();

        let name_from_cli = cli_args.ai_name.is_some();
        let name_from_env = std::env::var(ENV_CHAT_AI_NAME).is_ok();

        // Override if source was set at higher level than name
        if (source_from_cli && !name_from_cli)
            || (source_from_env && !name_from_env && !name_from_cli)
        {
            app_config.ai_name = Some(chat_source.default_ai_name().to_string());
        }
    }

    Ok(app_config)
}

pub fn build_config(cli_args: &CliArgs) -> Result<MarkdownConfig> {
    // First build the basic app config
    let app_config = build_app_config(cli_args)?;

    // Get the chat source
    let chat_source = app_config.chat_source.clone().unwrap();
    let ai_name = app_config.ai_name.clone().unwrap();

    // Then resolve paths and convert to markdown config
    let (_, input_path, output_path) = resolve_paths(
        &app_config.base_dir,
        app_config.inline_output,
        app_config.input_file.as_deref().unwrap(),
        app_config.output_file.as_deref(),
    )?;

    Ok(app_config.into_markdown_config(chat_source, ai_name, input_path, output_path))
}
