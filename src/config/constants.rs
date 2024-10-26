// Environment variables
pub const ENV_PREFIX: &str = "CHAT";
pub const ENV_SEPARATOR: &str = "_";
pub const ENV_CHAT_SOURCE: &str = "CHAT_SOURCE";
pub const ENV_CHAT_TIMEZONE: &str = "CHAT_TIMEZONE";
pub const ENV_CHAT_BASE_DIR: &str = "CHAT_BASE_DIR";
pub const ENV_CHAT_INLINE_OUTPUT: &str = "CHAT_INLINE_OUTPUT";
pub const ENV_CHAT_INPUT_FILE: &str = "CHAT_INPUT_FILE";
pub const ENV_CHAT_OUTPUT_FILE: &str = "CHAT_OUTPUT_FILE";
pub const ENV_CHAT_USER_NAME: &str = "CHAT_USER_NAME";
pub const ENV_CHAT_AI_NAME: &str = "CHAT_AI_NAME";
pub const ENV_CHAT_CONFIG_PATH: &str = "CHAT_CONFIG_PATH";

// Builder keys
pub const KEY_CHAT_SOURCE: &str = "chat_source";
pub const KEY_TIMEZONE: &str = "timezone";
pub const KEY_BASE_DIR: &str = "base_dir";
pub const KEY_INLINE_OUTPUT: &str = "inline_output";
pub const KEY_INPUT_FILE: &str = "input_file";
pub const KEY_OUTPUT_FILE: &str = "output_file";
pub const KEY_USER_NAME: &str = "user_name";
pub const KEY_AI_NAME: &str = "ai_name";

// Default values
pub const DEFAULT_CHAT_SOURCE: &str = "Claude";
pub const DEFAULT_TIMEZONE: &str = "UTC";
pub const DEFAULT_BASE_DIR: &str = "data";
pub const DEFAULT_INLINE_OUTPUT: bool = true;
pub const DEFAULT_USER_NAME: &str = "User";
pub const DEFAULT_AI_NAME: &str = "Assistant";
pub const DEFAULT_CHAT_CONFIG_PATH: &str = "config/app-settings.json";
