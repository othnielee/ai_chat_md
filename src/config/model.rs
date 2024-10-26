use super::types::ChatSource;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct AppConfig {
    pub chat_source: ChatSource,
    pub timezone: String,
    pub base_dir: String,
    pub inline_output: bool,
    pub input_file: String,
    pub output_file: Option<String>,
    pub user_name: String,
    pub ai_name: String,
}

pub struct MarkdownConfig {
    pub chat_source: ChatSource,
    pub timezone: String,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub user_name: String,
    pub ai_name: String,
}

impl AppConfig {
    pub fn into_markdown_config(self, input_path: PathBuf, output_path: PathBuf) -> MarkdownConfig {
        MarkdownConfig {
            chat_source: self.chat_source,
            timezone: self.timezone,
            input_file: input_path,
            output_file: output_path,
            user_name: self.user_name,
            ai_name: self.ai_name,
        }
    }
}
