#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Neat,
    Long,
    Cmd,
    Code
}


impl Mode {
    pub fn from_flag(flag: &str) -> Option<Mode> {
        if flag == "neat" || flag == "n" {
            return Some(Mode::Neat)
        } else if flag == "long" || flag == "l" {
            return Some(Mode::Long)
        } else if flag == "code" || flag == "c" {
            return Some(Mode::Code)
        } else if flag == "cmd" {
            return Some(Mode::Cmd)
        } else {
            return None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Prompt {
    pub mode: Mode,
    pub prompt: String,
    pub flags: Vec<String>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Prompt(Prompt),
    Wtf,
    InitCmd,
    NoCommand
}
