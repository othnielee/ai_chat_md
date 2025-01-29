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

#[derive(Debug)]
pub enum ChatGPTContentType {
    Text,
    UserEditableContext,
    Tool,
    System,
    MultimodalText,
    Unknown(String),
}

impl From<&str> for ChatGPTContentType {
    fn from(s: &str) -> Self {
        match s {
            "text" => ChatGPTContentType::Text,
            "user_editable_context" => ChatGPTContentType::UserEditableContext,
            "tool" => ChatGPTContentType::Tool,
            "system" => ChatGPTContentType::System,
            "multimodal_text" => ChatGPTContentType::MultimodalText,
            other => ChatGPTContentType::Unknown(other.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum DeepSeekContentType {
    Text,
    Thinking,
    Unknown(String),
}

impl From<&str> for DeepSeekContentType {
    fn from(s: &str) -> Self {
        // DeepSeek doesn't explicitly specify content types in the JSON
        // We'll infer based on the message properties
        match s {
            "text" => DeepSeekContentType::Text,
            "thinking" => DeepSeekContentType::Thinking,
            other => DeepSeekContentType::Unknown(other.to_string()),
        }
    }
}
