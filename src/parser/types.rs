pub(crate) enum ClaudeContentType {
    Text,
    ToolUse,
    ToolResult,
    Unknown(String),
}

impl From<&str> for ClaudeContentType {
    fn from(s: &str) -> Self {
        match s {
            "text" => ClaudeContentType::Text,
            "tool_use" => ClaudeContentType::ToolUse,
            "tool_result" => ClaudeContentType::ToolResult,
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
