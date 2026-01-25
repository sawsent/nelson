use serde::{Deserialize, Serialize};
use crate::domain::Mode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub backend: BackendSettings,
    pub llm: LlmSettings,
    pub nelson: NelsonSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackendSettings {
    pub provider: String,
    pub host: String,
    pub port: u16,
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
                host: "localhost".to_string(),
                port: 11434,
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


