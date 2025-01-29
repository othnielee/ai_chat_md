use super::types::ChatSource;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct AppConfig {
    pub chat_source: Option<ChatSource>,
    pub ai_name: Option<String>,
    pub user_name: String,
    pub title: Option<String>,
    pub timezone: String,
    pub reasoning: bool,
    pub base_dir: String,
    pub inline_output: bool,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
}

pub struct MarkdownConfig {
    pub chat_source: ChatSource,
    pub ai_name: String,
    pub user_name: String,
    pub title: Option<String>,
    pub timezone: String,
    pub reasoning: bool,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

impl AppConfig {
    pub fn into_markdown_config(
        self,
        chat_source: ChatSource,
        ai_name: String,
        input_path: PathBuf,
        output_path: PathBuf,
    ) -> MarkdownConfig {
        MarkdownConfig {
            chat_source,
            ai_name,
            user_name: self.user_name,
            title: self.title,
            timezone: self.timezone,
            reasoning: self.reasoning,
            input_file: input_path,
            output_file: output_path,
        }
    }
}
