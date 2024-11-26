// Config file environment variable
pub const ENV_APP_CONFIG_PATH: &str = "APP_CONFIG_PATH";

// Chat environment variable pattern
pub const ENV_PREFIX: &str = "CHAT";
pub const ENV_SEPARATOR: &str = "_";

// Chat environment variables
pub const ENV_CHAT_SOURCE: &str = "CHAT_SOURCE";
pub const ENV_CHAT_AI_NAME: &str = "CHAT_AI_NAME";
pub const ENV_CHAT_USER_NAME: &str = "CHAT_USER_NAME";
pub const ENV_CHAT_TITLE: &str = "CHAT_TITLE";
pub const ENV_CHAT_TIMEZONE: &str = "CHAT_TIMEZONE";
pub const ENV_CHAT_BASE_DIR: &str = "CHAT_BASE_DIR";
pub const ENV_CHAT_INLINE_OUTPUT: &str = "CHAT_INLINE_OUTPUT";
pub const ENV_CHAT_INPUT_FILE: &str = "CHAT_INPUT_FILE";
pub const ENV_CHAT_OUTPUT_FILE: &str = "CHAT_OUTPUT_FILE";

// Config builder keys
pub const KEY_CHAT_SOURCE: &str = "chat_source";
pub const KEY_AI_NAME: &str = "ai_name";
pub const KEY_USER_NAME: &str = "user_name";
pub const KEY_TITLE: &str = "title";
pub const KEY_TIMEZONE: &str = "timezone";
pub const KEY_BASE_DIR: &str = "base_dir";
pub const KEY_INLINE_OUTPUT: &str = "inline_output";
pub const KEY_INPUT_FILE: &str = "input_file";
pub const KEY_OUTPUT_FILE: &str = "output_file";

// Default config values
pub const DEFAULT_USER_NAME: &str = "User";
pub const DEFAULT_TIMEZONE: &str = "UTC";
pub const DEFAULT_BASE_DIR: &str = ".";
pub const DEFAULT_INLINE_OUTPUT: bool = true;
pub const DEFAULT_CONFIG_FILENAME: &str = ".aichatmd.json";
