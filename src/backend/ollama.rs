use crate::backend::Backend;
use crate::context::Context;
use crate::errors::NelsonError;
use crate::domain::Mode;
use serde::{Deserialize, Serialize};

pub struct OllamaBackend {
    url: String,
    model: String
}

impl OllamaBackend {
    pub fn new(url: &str, model: &str) -> Self {
        Self {
            url: url.to_string(),
            model: model.to_string()
        }
    }

    fn get_system_prompt(&self, mode: &Mode, model: &str) -> String {
        let base = "You are a Unix Command Line expert. You know the normal Unix commands, aswell as all the available tools that have been developed over the years. You are also an experienced developer in the languages Scala, Java, Python, and Bash. ";
        match (mode, model) {
            (Mode::Neat, _) => base.to_string() + "You are a concise responder. You always respond in a very concise manner, using concise language.",
            (Mode::Code, _) => base.to_string() + "You are a programming expert. You know all about Unix commands and the Unix terminal, aswell as all the available tools that have been developed over the years. You also have a very strong understanding of all programming languages. When you get a request for code, you only respond with the code itself needed to do what is asked. Nothing more. No explanations. No comments.",
            (Mode::Long, _) => base.to_string(),
            (Mode::Cmd, _) => base.to_string() + "When you get a request for a command, you only respond with the command itself needed to do what is asked. Nothing more. You don't wrap the command in markdown backticks, you simply return the command itself."
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
                OllamaMessage::new("system", &self.get_system_prompt(mode, &self.model)),
                OllamaMessage::new("user", prompt),
            ],
        };

        ctx.vprint(format_args!("Sending payload: {:?}, to url {}", payload, self.url));

        let response = client
            .post(self.url.clone())
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
                return Err(NelsonError::BackendUnreachable("ollama".to_string(), self.url.clone()));
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
