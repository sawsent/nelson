use crate::types::{Command, Mode, Prompt};
use std::collections::HashSet;

pub fn parse(args: &Vec<String>, default_mode: Mode) -> Command {
    match args.get(0) {
        Some(s) if s == "init" && has_only_flags(&args) => return Command::InitCmd,
        Some(s) if s == "wtf" && has_only_flags(&args) => return Command::Wtf,
        Some(_) => Command::Prompt(parse_prompt_args(&args, default_mode)),
        None => return Command::NoCommand,
    }
}

pub fn parse_flags(args: &Vec<String>) -> HashSet<String> {
    let mut flags: HashSet<String> = HashSet::new();
    for arg in args.get(1..).unwrap_or(&[]).to_vec() {
        match parse_flag_str(&arg) {
            Some(flag) => {
                flags.insert(flag.to_string());
            }
            None => (),
        }
    }
    return flags;
}

fn has_only_flags(args: &Vec<String>) -> bool {
    for arg in args.get(1..).unwrap_or(&[]).to_vec() {
        if let None = arg.strip_prefix("-") {
            return false;
        }
    }
    true
}

fn parse_prompt_args(args: &Vec<String>, default_mode: Mode) -> Prompt {
    let mut prompt_words: Vec<String> = Vec::new();
    let mut mode: Mode = default_mode;

    for arg in args {
        match parse_flag_str(arg) {
            Some(f) => {
                if let Some(m) = Mode::from_string(f) {
                    mode = m;
                }
            }
            None => prompt_words.push(arg.to_string()),
        }
    }
    Prompt {
        prompt: prompt_words.join(" ").to_string(),
        mode: mode,
    }
}

fn parse_flag_str(arg: &String) -> Option<&str> {
    if let Some(flag) = arg.strip_prefix("--") {
        return Some(flag);
    } else if let Some(flag) = arg.strip_prefix("-") {
        return Some(flag);
    } else {
        return None;
    }
}
