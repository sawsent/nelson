use crate::backend::Backend;
use crate::context::Context;
use crate::errors::NelsonError;
use serde::{Deserialize, Serialize};

const OLLAMA_TYPE: &'static str = "ollama";

pub struct OllamaBackend {
    host: String,
    port: u16,
}

impl OllamaBackend {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port: port,
        }
    }
}

impl Backend for OllamaBackend {
    fn get_type() -> &'static str { OLLAMA_TYPE }
    fn query(&self, system_prompt: &str, user_prompt: &str, model_name: &str, ctx: &Context) -> Result<String, NelsonError> {
        let client = reqwest::blocking::Client::new();
        let payload = OllamaChatRequest {
            model: model_name.to_string(),
            stream: false,
            messages: vec![
                OllamaMessage::new("system", system_prompt),
                OllamaMessage::new("user", user_prompt),
            ],
        };

        ctx.vprint(format_args!("Sending payload: {:?}", payload));


        let response = client
            .post(format!("http://{}:{}/api/chat", self.host, self.port))
            .json(&payload)
            .send();

        match response {
            Ok(resp) => {
                let status = resp.status();

                if !status.is_success() {
                    return Err(NelsonError::Http(status.as_u16()));
                }

                let body: Result<OllamaChatResponse, reqwest::Error> = resp.json();

                match body {
                    Ok(data) => {
                        let text = data.message.content;
                        if text.trim().is_empty() {
                            return Err(NelsonError::EmptyResponse);
                        }
                        return Ok(text);
                    }
                    Err(_) => {
                        return Err(NelsonError::InvalidResponse);
                    }
                }
            }
            Err(_) => {
                return Err(NelsonError::BackendUnreachable(self.host.clone(), self.port));
            }
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
            content: content.to_string()
        }
    }
}
