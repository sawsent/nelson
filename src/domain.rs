use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Neat,
    Long,
    Cmd,
    Code,
}

impl Mode {
    pub fn from_string(flag: &str) -> Option<Mode> {
        match flag {
            "--neat" | "-n" => Some(Mode::Neat),
            "--long" | "-l" => Some(Mode::Long),
            "--code" | "-c" => Some(Mode::Code),
            "--cmd" => Some(Mode::Cmd),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Prompt {
    pub mode: Mode,
    pub prompt: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Init {}

#[derive(Debug, PartialEq, Eq)]
pub struct Wtf {}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Prompt(Prompt),
    WtfCmd(Wtf),
    InitCmd(Init),
    NoCmd,
}
