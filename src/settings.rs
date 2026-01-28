use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use crate::domain::Mode;
use crate::context::Flag;

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
    pub fallback_urls: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmSettings {
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NelsonSettings {
    pub default_mode: Mode,
}

impl Settings {
    fn apply_flag(&mut self, flag: &Flag) {
        match flag {
            Flag::Provider(p) => {
                self.backend.provider = p.clone();
                if let Some(fallback) = self.backend.fallback_urls.get(p) {
                    self.backend.url = fallback.to_string();
                }
            }
            Flag::Model(m) => self.llm.model = m.clone(),
            _ => {}
        }
    }
    pub fn with_flags(&self, flags: &HashSet<Flag>) -> Self {
        let mut base = self.clone();
        for flag in flags {
            base.apply_flag(flag)
        }
        base
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            backend: BackendSettings {
                provider: "ollama".to_string(),
                url: "http://localhost:11434/api/chat".to_string(),
                token: None,
                fallback_urls: HashMap::new()
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

