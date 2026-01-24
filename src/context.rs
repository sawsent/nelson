use std::collections::HashSet;
use std::fmt::Arguments;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Flag {
    Verbose,
    Help
}

impl Flag {
    pub fn from_string(s: &str) -> Option<Flag> {
        match s {
            "--verbose" => Some(Flag::Verbose),
            "--help" | "-h" => Some(Flag::Help),
            _ => None
        }
    }
}

pub struct ContextBuilder {
    pub full_cmd: String,
    pub flags: HashSet<Flag>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            full_cmd: "".to_string(),
            flags: HashSet::new(),
        }
    }
    pub fn set_cmd(&mut self, s: &str) {
        self.full_cmd = s.to_string();
    }
    pub fn add_flags(&mut self, flags: &HashSet<Flag>) {
        for flag in flags {
            self.flags.insert(flag.to_owned());
        }
    }
    pub fn build(&self) -> Context {
        Context {
            full_cmd: self.full_cmd.clone(),
            _flags: self.flags.clone(),
            verbose: self.flags.contains(&Flag::Verbose),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    full_cmd: String,
    _flags: HashSet<Flag>,
    verbose: bool,
}

impl Context {
    pub fn vprint(&self, args: Arguments) {
        if self.verbose {
            eprintln!("[verbose] {}", args);
        }
    }
}
