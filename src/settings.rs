use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use toml;

use crate::types::Mode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub llm: LlmSettings,
    pub prompt: PromptSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptSettings {
    pub default_mode: Mode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            llm: LlmSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            prompt: PromptSettings {
                default_mode: Mode::Neat,
            },
        }
    }
}

impl Settings {
    pub fn load(fp: &PathBuf) -> Settings {
        match fs::read_to_string(fp) {
            Ok(contents) => {
                toml::from_str(&contents).unwrap_or_else(|err| {
                    eprintln!(
                        "nelson: failed to parse config at {:?}: {}, using default.",
                        fp, err
                    );
                    Self::default()
                })
            },
            Err(e) => {
                println!("{:?}", e);
                Self::default()
            }
        }
    }
}
