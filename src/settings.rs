use crate::domain::Mode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub backend: BackendSettings,
    pub llm: LlmSettings,
    pub nelson: NelsonSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendSettings {
    pub provider: String,
    pub url: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmSettings {
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NelsonSettings {
    pub default_mode: Mode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            backend: BackendSettings {
                provider: "ollama".to_string(),
                url: "http://localhost:11434/api/chat".to_string(),
                token: None,
            },
            llm: LlmSettings {
                model: "llama3.2".to_string(),
            },
            nelson: NelsonSettings {
                default_mode: Mode::Cmd,
            },
        }
    }
}
