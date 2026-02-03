use crate::context::Flag;
use crate::domain::{Command, Init, Mode, Prompt, Wtf};
use std::collections::HashSet;

pub fn parse_args(args: &Vec<String>, default_mode: &Mode) -> Command {
    match args.first() {
        Some(s) if s == "init" && has_only_flags(args) => Command::InitCmd(Init {}),
        Some(s) if s == "wtf" && has_only_flags(args) => Command::WtfCmd(Wtf {}),
        Some(_) => Command::Prompt(parse_prompt_args(args, default_mode)),
        None => Command::NoCmd,
    }
}

pub fn split_args_and_flags(args: &Vec<String>) -> (Vec<String>, HashSet<Flag>) {
    let mut flags: HashSet<Flag> = HashSet::new();
    let mut args_without_flags: Vec<String> = Vec::new();
    for arg in args {
        match parse_flag(arg) {
            Some(flag) => {
                flags.insert(flag);
            }
            None => args_without_flags.push(arg.to_string()),
        }
    }
    (args_without_flags, flags)
}

fn has_only_flags(args: &[String]) -> bool {
    for arg in args.get(1..).unwrap_or(&[]).to_vec() {
        if parse_flag(&arg).is_none() {
            return false;
        }
    }
    true
}

fn parse_prompt_args(args: &Vec<String>, default_mode: &Mode) -> Prompt {
    let mut prompt_words: Vec<String> = Vec::new();
    let mut mode: Mode = default_mode.clone();

    for arg in args {
        if let Some(m) = Mode::from_string(arg) {
            mode = m
        } else {
            match parse_flag(arg) {
                Some(_) => (),
                None => prompt_words.push(arg.to_string()),
            }
        }
    }
    Prompt {
        prompt: prompt_words.join(" ").to_string(),
        mode,
    }
}

fn parse_flag(arg: &str) -> Option<Flag> {
    Flag::from_string(arg)
}
