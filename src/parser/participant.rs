use crate::config::MarkdownConfig;

pub(crate) struct ClaudeParticipantMapper<'a> {
    config: &'a MarkdownConfig,
}

impl<'a> ClaudeParticipantMapper<'a> {
    pub fn new(config: &'a MarkdownConfig) -> Self {
        Self { config }
    }

    pub fn get_name<'b>(&'b self, sender: &'b str) -> &'b str {
        match sender {
            "human" => &self.config.user_name,
            "assistant" => &self.config.ai_name,
            other => other,
        }
    }
}

pub(crate) struct ChatGPTParticipantMapper<'a> {
    config: &'a MarkdownConfig,
}

impl<'a> ChatGPTParticipantMapper<'a> {
    pub fn new(config: &'a MarkdownConfig) -> Self {
        Self { config }
    }

    pub fn get_name<'b>(&'b self, sender: &'b str) -> &'b str {
        match sender {
            "user" => &self.config.user_name,
            "assistant" => &self.config.ai_name,
            "system" => &self.config.ai_name,
            "tool" => &self.config.ai_name,
            other => other,
        }
    }
}

pub(crate) struct DeepSeekParticipantMapper<'a> {
    config: &'a MarkdownConfig,
}

impl<'a> DeepSeekParticipantMapper<'a> {
    pub fn new(config: &'a MarkdownConfig) -> Self {
        Self { config }
    }

    pub fn get_name<'b>(&'b self, sender: &'b str) -> &'b str {
        match sender {
            "USER" => &self.config.user_name,
            "ASSISTANT" => &self.config.ai_name,
            other => other,
        }
    }
}
