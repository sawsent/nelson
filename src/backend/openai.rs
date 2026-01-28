use crate::backend::Backend;
use crate::context::Context;
use crate::errors::NelsonError;
use crate::domain::Mode;
use crate::backend::BackendAuth;
use serde::{Deserialize, Serialize};

pub struct OpenAiBackend {
    url: String,
    auth: BackendAuth,
    model: String
}

impl OpenAiBackend {
    pub fn new(url: &str, auth: BackendAuth, model: &str) -> Self {
        Self {
            url: url.to_string(),
            auth: auth,
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

impl Backend for OpenAiBackend {
    fn query(&self, prompt: &str, mode: &Mode, ctx: &Context) -> Result<String, NelsonError> {
        let client = reqwest::blocking::Client::new();
        let payload = OpenaiChatRequest {
            model: self.model.clone(),
            stream: false,
            messages: vec![
                OpenAiMessage::new("system", &self.get_system_prompt(mode, &self.model)),
                OpenAiMessage::new("user", prompt),
            ],
        };

        ctx.vprint(format_args!("Sending payload: {:?}, to url {}", payload, self.url));

        let mut builder = client
            .post(self.url.clone())
            .json(&payload);

        match &self.auth {
            BackendAuth::None => (),
            BackendAuth::Token(tkn) => builder = builder.header("Authorization", format!("Bearer {}", tkn)),
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
                    ctx.vprint(format_args!("Error response: {:?}", resp));
                    return Err(NelsonError::Http(status.as_u16()));
                }

                let body: Result<OpenAiChatResponse, reqwest::Error> = resp.json();

                match body {
                    Ok(data) => {
                        let maybe_text = data.choices.get(0).map(|choice| choice.message.content.clone());
                        match maybe_text {
                            None => Err(NelsonError::EmptyResponse),
                            Some(t) if t.trim().is_empty() => Err(NelsonError::EmptyResponse),
                            Some(t) => Ok(t)
                        }
                    }
                    Err(err) => {
                        Err(NelsonError::InvalidResponse(format!("{}", err)))
                    }
                }
            }
            Err(_) => {
                Err(NelsonError::BackendUnreachable("ollama".to_string(), self.url.clone()))
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenaiChatRequest {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<OpenAiMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<OpenAiChatChoice>,
    pub usage: Option<OpenAiUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiChatChoice {
    pub index: u32,
    pub message: OpenAiMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    pub role: String,
    pub content: String,
}

impl OpenAiMessage {
    pub fn new(role: &'static str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string()
        }
    }
}
