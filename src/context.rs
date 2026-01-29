use std::collections::HashSet;
use std::fmt::Arguments;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Flag {
    Verbose,
    Help,
    Provider(String),
    Model(String),
    Strict
}

impl Flag {
    pub fn from_string(s: &str) -> Option<Flag> {
        match s {
            "--verbose" => Some(Flag::Verbose),
            "--help" | "-h" => Some(Flag::Help),
            "--strict" => Some(Flag::Strict),
            _ => Self::indirect_matches(s),
        }
    }
    fn indirect_matches(s: &str) -> Option<Flag> {
        if let Some(model) = s.strip_prefix("--model=") {
            Some(Self::Model(model.to_string()))
        } else if let Some(provider) = s.strip_prefix("--provider=") {
            Some(Self::Provider(provider.to_string()))
        } else {
            None
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
            _full_cmd: self.full_cmd.clone(),
            flags: self.flags.clone(),
            verbose: self.flags.contains(&Flag::Verbose),
            strict: self.flags.contains(&Flag::Strict),
            is_help: self.flags.contains(&Flag::Help),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    _full_cmd: String,
    pub flags: HashSet<Flag>,
    verbose: bool,
    pub strict: bool,
    pub is_help: bool,
}

impl Context {
    pub fn vprint(&self, args: Arguments) {
        if self.verbose {
            eprintln!("[verbose] {}", args);
        }
    }
}
