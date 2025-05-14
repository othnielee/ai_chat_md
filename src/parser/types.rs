pub(crate) enum ClaudeContentType {
    Text,
    Thinking,
    ToolUse,
    ToolResult,
    Unknown(String),
}

impl From<&str> for ClaudeContentType {
    fn from(s: &str) -> Self {
        match s {
            "text" => ClaudeContentType::Text,
            "thinking" => ClaudeContentType::Thinking,
            "tool_use" => ClaudeContentType::ToolUse,
            "tool_result" => ClaudeContentType::ToolResult,
            other => ClaudeContentType::Unknown(other.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum ChatGPTContentType {
    Text,
    TetherQuote,
    MultimodalText,
    UserEditableContext,
    Tool,
    System,
    Code,
    Unknown(String),
}

impl From<&str> for ChatGPTContentType {
    fn from(s: &str) -> Self {
        match s {
            "text" => ChatGPTContentType::Text,
            "tether_quote" => ChatGPTContentType::TetherQuote,
            "multimodal_text" => ChatGPTContentType::MultimodalText,
            "user_editable_context" => ChatGPTContentType::UserEditableContext,
            "tool" => ChatGPTContentType::Tool,
            "system" => ChatGPTContentType::System,
            "code" => ChatGPTContentType::Code,
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
