use super::ConfigError;
use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ChatSource {
    #[value(name = "claude")]
    Claude,
    #[value(name = "chatgpt")]
    ChatGPT,
}

impl ChatSource {
    pub fn platform_name(&self) -> &'static str {
        match self {
            ChatSource::Claude => "Claude",
            ChatSource::ChatGPT => "ChatGPT",
        }
    }

    pub fn default_ai_name(&self) -> &'static str {
        match self {
            ChatSource::Claude => "Claude",
            ChatSource::ChatGPT => "ChatGPT",
        }
    }
}

impl std::str::FromStr for ChatSource {
    type Err = ConfigError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "claude" => Ok(ChatSource::Claude),
            "chatgpt" => Ok(ChatSource::ChatGPT),
            _ => Err(ConfigError::ChatSource(format!(
                "Invalid chat source: {}",
                s
            ))),
        }
    }
}
