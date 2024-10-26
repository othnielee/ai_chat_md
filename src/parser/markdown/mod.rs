mod chatgpt;
mod claude;

pub use chatgpt::parse_to_markdown as parse_chatgpt_to_markdown;
pub use claude::parse_to_markdown as parse_claude_to_markdown;
