use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChatGPTChat {
    pub title: String,
    #[serde(rename = "create_time")]
    pub created_at: f64,
    #[serde(rename = "update_time")]
    pub updated_at: f64,
    pub mapping: std::collections::HashMap<String, ChatGPTNode>,
    pub current_node: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ChatGPTNode {
    pub id: String,
    pub message: Option<ChatGPTMessage>,
    pub parent: Option<String>,
    pub children: Vec<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ChatGPTMessage {
    pub id: String,
    pub author: ChatGPTAuthor,
    #[serde(rename = "create_time")]
    pub created_at: Option<f64>,
    pub content: ChatGPTContent,
    pub status: String,
    #[serde(default)]
    pub metadata: ChatGPTMetadata,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ChatGPTAuthor {
    pub role: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ChatGPTContent {
    pub content_type: String,
    #[serde(default)]
    pub parts: Vec<String>,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Default, Deserialize)]
#[allow(dead_code)]
pub struct ChatGPTMetadata {
    #[serde(default)]
    pub is_visually_hidden_from_conversation: bool,
}
