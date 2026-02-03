use crate::backend::Backend;
use crate::backend::BackendAuth;
use crate::context::Context;
use crate::domain::Mode;
use crate::errors::NelsonError;
use crate::r#static::system_prompts;
use serde::{Deserialize, Serialize};

pub struct OllamaBackend {
    url: String,
    auth: BackendAuth,
    model: String,
}

impl OllamaBackend {
    pub fn new(url: &str, auth: BackendAuth, model: &str) -> Self {
        Self {
            url: url.to_string(),
            auth,
            model: model.to_string(),
        }
    }
}

impl Backend for OllamaBackend {
    fn query(&self, prompt: &str, mode: &Mode, ctx: &Context) -> Result<String, NelsonError> {
        let client = reqwest::blocking::Client::new();
        let payload = OllamaChatRequest {
            model: self.model.clone(),
            stream: false,
            messages: vec![
                OllamaMessage::new(
                    "system",
                    &system_prompts::get_system_prompt(mode, ctx.strict),
                ),
                OllamaMessage::new("user", prompt),
            ],
        };

        ctx.vprint(format_args!(
            "Sending payload: {:?}, to url {}",
            payload, self.url
        ));

        let mut builder = client.post(self.url.clone()).json(&payload);

        match &self.auth {
            BackendAuth::None => (),
            BackendAuth::Token(tkn) => {
                builder = builder.header("Authorization", format!("Bearer {}", tkn))
            }
        }

        let response = builder.send();

        ctx.vprint(format_args!("Got response: {:?}", response));

        match response {
            Ok(resp) => {
                let status = resp.status();

                if !status.is_success() {
                    let s = status.as_u16();
                    if s == 404 {
                        return Err(NelsonError::ModelError(self.model.clone()));
                    }
                    return Err(NelsonError::Http(status.as_u16()));
                }

                let body: Result<OllamaChatResponse, reqwest::Error> = resp.json();

                match body {
                    Ok(data) => {
                        let text = data.message.content;
                        if text.trim().is_empty() {
                            return Err(NelsonError::EmptyResponse);
                        }
                        Ok(text)
                    }
                    Err(err) => Err(NelsonError::InvalidResponse(format!("{}", err))),
                }
            }
            Err(_) => Err(NelsonError::BackendUnreachable(
                "ollama".to_string(),
                self.url.clone(),
            )),
        }
    }
}

#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<OllamaMessage>,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    pub message: OllamaMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    pub role: String,
    pub content: String,
}

impl OllamaMessage {
    pub fn new(role: &'static str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}
