use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

use crate::context::Context;
use crate::types::Mode;

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
                model: "llama3.2".to_string()
            },
            nelson: NelsonSettings {
                default_mode: Mode::Cmd,
            }
        }
    }
}

pub fn load(fp: &PathBuf, default: Settings, ctx: &Context) -> Settings {
    match fs::read_to_string(fp) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_else(|err| {
            ctx.vprint(format_args!(
                "nelson: failed to parse config at {:?}. Error: {}. Using default.",
                fp, err
            ));
            default
        }),
        Err(e) => {
            ctx.vprint(format_args!(
                "Could not access the file at {:?}. Error: {:?}. Using default.",
                fp, e
            ));
            default
        }
    }
}
