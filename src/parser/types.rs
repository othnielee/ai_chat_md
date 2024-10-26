pub(crate) enum ClaudeContentType {
    Text,
    Unknown(String),
}

impl From<&str> for ClaudeContentType {
    fn from(s: &str) -> Self {
        match s {
            "text" => ClaudeContentType::Text,
            other => ClaudeContentType::Unknown(other.to_string()),
        }
    }
}

pub(crate) enum ChatGPTContentType {
    Text,
    UserEditableContext,
    Tool,
    System,
    Unknown(String),
}

impl From<&str> for ChatGPTContentType {
    fn from(s: &str) -> Self {
        match s {
            "text" => ChatGPTContentType::Text,
            "user_editable_context" => ChatGPTContentType::UserEditableContext,
            "tool" => ChatGPTContentType::Tool,
            "system" => ChatGPTContentType::System,
            other => ChatGPTContentType::Unknown(other.to_string()),
        }
    }
}
