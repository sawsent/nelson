use crate::backend::Backend;
use crate::context::Context;
use crate::errors::NelsonError;
use crate::domain::Mode;
use serde::{Deserialize, Serialize};

pub struct OllamaBackend {
    host: String,
    port: Option<u16>,
    model: String
}

impl OllamaBackend {
    pub fn new(host: &str, port: u16, model: &str) -> Self {
        Self {
            host: host.to_string(),
            port: Some(port),
            model: model.to_string()
        }
    }

    fn get_system_prompt(&self, mode: &Mode) -> String {
        match mode {
            Mode::Cmd => "".to_string(),
            Mode::Neat => "".to_string(),
            Mode::Long => "".to_string(),
            Mode::Code => "".to_string()
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
                OllamaMessage::new("system", &self.get_system_prompt(mode)),
                OllamaMessage::new("user", prompt),
            ],
        };

        let url = self.port
            .map(|p| format!("http://{}:{}/api/chat", self.host, p))
            .unwrap_or_else(|| format!("http://{}/api/chat", self.host));

        ctx.vprint(format_args!("Sending payload: {:?}, to url {}", payload, url));

        let response = client
            .post(url)
            .json(&payload)
            .send();

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
                        return Ok(text);
                    }
                    Err(err) => {
                        return Err(NelsonError::InvalidResponse(format!("{}", err)));
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
