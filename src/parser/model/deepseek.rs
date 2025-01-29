use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeepSeekResponse {
    pub code: i32,
    pub msg: String,
    pub data: DeepSeekResponseData,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeepSeekResponseData {
    pub biz_code: i32,
    pub biz_msg: String,
    pub biz_data: DeepSeekChat,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeepSeekChat {
    pub chat_session: DeepSeekSession,
    pub chat_messages: Vec<DeepSeekMessage>,
    pub cache_valid: bool,
    pub route_id: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeepSeekSession {
    pub id: String,
    pub seq_id: i64,
    pub agent: String,
    pub character: Option<String>,
    pub title: String,
    pub title_type: String,
    pub version: i32,
    pub current_message_id: i64,
    pub inserted_at: f64,
    pub updated_at: f64,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeepSeekMessage {
    pub message_id: i64,
    pub parent_id: Option<i64>,
    pub model: String,
    pub role: String,
    pub content: String,
    pub thinking_enabled: bool,
    pub thinking_content: Option<String>,
    pub thinking_elapsed_secs: Option<f64>,
    pub ban_edit: bool,
    pub ban_regenerate: bool,
    pub status: String,
    pub accumulated_token_usage: i64,
    pub files: Vec<DeepSeekFile>,
    pub inserted_at: f64,
    pub search_enabled: bool,
    pub search_status: Option<String>,
    pub search_results: Option<String>,
    pub tip: Option<String>,
    pub feedback: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeepSeekFile {
    pub id: String,
    pub status: String,
    pub file_name: String,
    pub file_size: i64,
    pub token_usage: i64,
    pub error_code: Option<String>,
    pub inserted_at: f64,
    pub updated_at: f64,
}
