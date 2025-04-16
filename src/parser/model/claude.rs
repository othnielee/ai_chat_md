use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ClaudeChat {
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub chat_messages: Vec<ClaudeMessage>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ClaudeMessage {
    pub uuid: String,
    pub text: String,
    pub index: u32,
    pub sender: String,
    pub created_at: String,
    pub content: Vec<ClaudeContent>,
    pub attachments: Vec<ClaudeAttachment>,
    pub updated_at: String,
    pub truncated: bool,
    #[serde(default)]
    pub stop_reason: Option<String>,
    #[serde(default)]
    pub files: Vec<File>,
    #[serde(default)]
    pub files_v2: Vec<FileV2>,
    #[serde(default)]
    pub sync_sources: Vec<String>,
    pub parent_message_uuid: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub start_timestamp: String,
    pub stop_timestamp: String,
    pub text: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "input")]
    pub artifact: Option<ClaudeArtifact>,
    pub thinking: Option<String>,
    pub summaries: Option<Vec<ClaudeSummary>>,
}

#[derive(Deserialize)]
pub struct ClaudeSummary {
    pub summary: String,
}

#[derive(Deserialize)]
pub struct ClaudeArtifact {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Deserialize)]
pub struct ClaudeAttachment {
    pub file_name: String,
    pub extracted_content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub file_kind: String,
    pub file_uuid: String,
    pub file_name: String,
    pub created_at: String,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub preview_url: Option<String>,
    #[serde(default)]
    pub thumbnail_asset: Option<ThumbnailAsset>,
    #[serde(default)]
    pub preview_asset: Option<PreviewAsset>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileV2 {
    pub file_kind: String,
    pub file_uuid: String,
    pub file_name: String,
    pub created_at: String,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub preview_url: Option<String>,
    #[serde(default)]
    pub thumbnail_asset: Option<ThumbnailAsset>,
    #[serde(default)]
    pub preview_asset: Option<PreviewAsset>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThumbnailAsset {
    pub url: String,
    pub file_variant: String,
    pub primary_color: String,
    pub image_width: u32,
    pub image_height: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PreviewAsset {
    pub url: String,
    pub file_variant: String,
    pub primary_color: String,
    pub image_width: u32,
    pub image_height: u32,
}
