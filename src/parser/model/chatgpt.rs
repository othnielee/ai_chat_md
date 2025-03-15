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
    pub title: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub parts: Vec<ChatGPTContentPart>,
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Default, Deserialize)]
#[allow(dead_code)]
pub struct ChatGPTMetadata {
    #[serde(default)]
    pub model_slug: Option<String>,
    #[serde(default)]
    pub default_model_slug: Option<String>,
    #[serde(default)]
    pub initial_text: Option<String>,
    #[serde(default)]
    pub finished_text: Option<String>,
    #[serde(default)]
    pub is_visually_hidden_from_conversation: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum ChatGPTContentPart {
    Text(String),
    ImageAssetPointer(ImageAssetPointer),
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ImageAssetPointer {
    pub content_type: String,
    pub asset_pointer: String,
    pub size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub fovea: Option<String>,
    pub metadata: ImageMetadata,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ImageMetadata {
    pub dalle: Option<String>,
    pub gizmo: Option<String>,
    pub emu_omit_glimpse_image: Option<String>,
    pub emu_patches_override: Option<String>,
    pub sanitized: bool,
}
