use crate::backend::Backend;
use serde::{Deserialize, Serialize};

struct OllamaBackend {
    host: String,
    port: u16,
}

impl Backend for OllamaBackend {
    fn query(&self, system_prompt: &str, user_prompt: &str) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
}

#[derive(Debug, Deserialize)]
pub struct OllamaChatResponse {
    pub message: OllamaMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}
