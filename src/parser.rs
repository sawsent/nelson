use crate::types::{Command, Mode, Prompt};
use crate::context::Flag;
use std::collections::HashSet;

pub fn parse_args(args: &Vec<String>, default_mode: Mode) -> Command {
    match args.get(0) {
        Some(s) if s == "init" && has_only_flags(&args) => return Command::InitCmd,
        Some(s) if s == "wtf" && has_only_flags(&args) => return Command::Wtf,
        Some(_) => Command::Prompt(parse_prompt_args(&args, default_mode)),
        None => return Command::NoCommand,
    }
}

pub fn split_args_and_flags(args: &Vec<String>) -> (Vec<String>, HashSet<Flag>) {
    let mut flags: HashSet<Flag> = HashSet::new();
    let mut args_without_flags: Vec<String> = Vec::new();
    for arg in args.get(1..).unwrap_or(&[]).to_vec() {
        match parse_flag(&arg) {
            Some(flag) => {
                flags.insert(flag);
            }
            None => args_without_flags.push(arg)
        }
    }
    return (args_without_flags, flags);
}

fn has_only_flags(args: &Vec<String>) -> bool {
    for arg in args.get(1..).unwrap_or(&[]).to_vec() {
        if let None = parse_flag(&arg) {
            return false;
        }
    }
    true
}

fn parse_prompt_args(args: &Vec<String>, default_mode: Mode) -> Prompt {
    let mut prompt_words: Vec<String> = Vec::new();
    let mut mode: Mode = default_mode;

    for arg in args {
        if let Some(m) = Mode::from_string(arg) {
            mode = m
        } else {
            match parse_flag(arg) {
                Some(_) => (),
                None => prompt_words.push(arg.to_string())
            }
        }
    }
    Prompt {
        prompt: prompt_words.join(" ").to_string(),
        mode: mode,
    }
}

fn parse_flag(arg: &String) -> Option<Flag> {
    if let Some(flag) = Flag::from_string(arg) {
        return Some(flag);
    } else {
        return None;
    }
}
