use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

use crate::context::Context;
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
            }
        }
    }
}

impl Settings {
    pub fn load(fp: &PathBuf, ctx: &Context) -> Settings {
        match fs::read_to_string(fp) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_else(|err| {
                ctx.vprint(format_args!(
                    "nelson: failed to parse config at {:?}. Error: {}. Using default.",
                    fp, err
                ));
                Self::default()
            }),
            Err(e) => {
                ctx.vprint(format_args!(
                    "Could not access the file at {:?}. Error: {:?}. Using default.",
                    fp, e
                ));
                Self::default()
            }
        }
    }
}
