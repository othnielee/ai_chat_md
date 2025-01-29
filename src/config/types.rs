use super::ConfigError;
use clap::{builder::PossibleValue, ValueEnum};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatSource {
    Claude,
    ChatGPT,
    DeepSeek,
}

impl ValueEnum for ChatSource {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Claude, Self::ChatGPT, Self::DeepSeek]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Claude => PossibleValue::new("claude"),
            Self::ChatGPT => PossibleValue::new("chatgpt"),
            Self::DeepSeek => PossibleValue::new("deepseek"),
        })
    }

    // Implement from_str for ValueEnum
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let converted = if ignore_case {
            input.to_lowercase()
        } else {
            input.to_string()
        };

        match converted.as_str() {
            "claude" => Ok(Self::Claude),
            "chatgpt" => Ok(Self::ChatGPT),
            "deepseek" => Ok(Self::DeepSeek),
            _ => Err(format!("Invalid chat source: {}", input)),
        }
    }
}

impl std::str::FromStr for ChatSource {
    type Err = ConfigError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "claude" => Ok(ChatSource::Claude),
            "chatgpt" => Ok(ChatSource::ChatGPT),
            "deepseek" => Ok(ChatSource::DeepSeek),
            _ => Err(ConfigError::ChatSource(format!(
                "Invalid chat source: {}",
                s
            ))),
        }
    }
}

impl ChatSource {
    pub fn platform_name(&self) -> &'static str {
        match self {
            ChatSource::Claude => "Claude",
            ChatSource::ChatGPT => "ChatGPT",
            ChatSource::DeepSeek => "DeepSeek",
        }
    }

    pub fn default_ai_name(&self) -> &'static str {
        match self {
            ChatSource::Claude => "Claude",
            ChatSource::ChatGPT => "ChatGPT",
            ChatSource::DeepSeek => "DeepSeek",
        }
    }
}
