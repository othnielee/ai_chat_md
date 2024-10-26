use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClaudeChat {
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub chat_messages: Vec<ClaudeMessage>,
}

#[derive(Deserialize)]
pub struct ClaudeMessage {
    pub sender: String,
    pub created_at: String,
    pub content: Vec<ClaudeContent>,
    pub attachments: Vec<ClaudeAttachment>,
}

#[derive(Deserialize)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct ClaudeAttachment {
    pub file_name: String,
    pub extracted_content: String,
}
