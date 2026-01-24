use crate::context::Context;
use crate::domain::{Mode, Prompt};
use crate::settings::{BackendSettings, Settings};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}
impl Message {
    pub fn new(role: &'static str, content: &String) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

pub enum NelsonError {
    Unexpected,
}

pub fn query_prompt(
    prompt: &Prompt,
    settings: &Settings,
    ctx: &Context,
) -> Result<String, reqwest::Error> {
    let system_prompt = get_system_prompt(&prompt.mode);
    ctx.vprint(format_args!("Using system prompt: {}", system_prompt));

    ctx.vprint(format_args!("Querying LLM: {}", system_prompt));
    let response = query(&system_prompt, &prompt.prompt, settings, ctx)?;
    println!("{:#?}", response);

    Ok(response)
}

fn query(
    system_prompt: &String,
    prompt: &String,
    settings: &Settings,
    ctx: &Context,
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let payload = Payload {
        model: "stable-code".to_string(),
        stream: false,
        messages: vec![
            Message::new("system", system_prompt),
            Message::new("user", prompt),
        ],
    };

    ctx.vprint(format_args!("Sending payload: {:?}", payload));

    let response = client
        .post(get_url(&settings.backend))
        .json(&payload)
        .send()?;

    let body: serde_json::Value = response.json()?;

    println!("{:#?}", body);
    Ok("".to_string())
}

fn get_url(settings: &BackendSettings) -> String {
    format!("http://{}:{}/api/chat", settings.host, settings.port)
}

fn get_system_prompt(mode: &Mode) -> String {
    match mode {
        Mode::Cmd => "".to_string(),
        Mode::Code => "".to_string(),
        Mode::Neat => "".to_string(),
        Mode::Long => "".to_string(),
    }
}
