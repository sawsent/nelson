use crate::backend::ollama::OllamaBackend;
use crate::backend::openai::OpenAiBackend;
use crate::context::Context;
use crate::domain::Mode;
use crate::errors::NelsonError;
use crate::settings::{BackendSettings, LlmSettings, Settings};

#[derive(Clone)]
pub enum BackendAuth {
    None,
    Token(String),
}

impl BackendAuth {
    pub fn from_settings(token: &Option<String>) -> Self {
        match token {
            None => BackendAuth::None,
            Some(t) if t.trim().is_empty() => BackendAuth::None,
            Some(t) => BackendAuth::Token(t.to_string()),
        }
    }
}

pub trait Backend {
    fn query(&self, prompt: &str, mode: &Mode, ctx: &Context) -> Result<String, NelsonError>;
}

pub fn default_backend() -> Box<dyn Backend> {
    let ds = Settings::default();
    Box::new(OllamaBackend::new(
        &ds.backend.url,
        BackendAuth::None,
        &ds.llm.model,
    ))
}

pub fn backend_from_settings(
    backend_settings: &BackendSettings,
    llm_settings: &LlmSettings,
) -> Result<Box<dyn Backend>, NelsonError> {
    match backend_settings.provider.as_str() {
        "openai" => Ok(Box::new(OpenAiBackend::new(
            &backend_settings.url,
            BackendAuth::from_settings(&backend_settings.token),
            &llm_settings.model,
        ))),
        "ollama" => Ok(Box::new(OllamaBackend::new(
            &backend_settings.url,
            BackendAuth::from_settings(&backend_settings.token),
            &llm_settings.model,
        ))),
        other => Err(NelsonError::Internal(format!(
            "Unknown backend provider: {}",
            other
        ))),
    }
}

pub mod ollama;
pub mod openai;
