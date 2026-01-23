use std::collections::HashSet;
use std::fmt::Arguments;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Flag {
    Verbose
}

impl Flag {
    pub fn from_string(s: &str) -> Option<Flag> {
        match s {
            "--verbose" => Some(Flag::Verbose),
            _ => None
        }
    }
}

pub struct ContextBuilder {
    pub flags: HashSet<Flag>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
        }
    }
    pub fn add_flags(&mut self, flags: &HashSet<Flag>) {
        for flag in flags {
            self.flags.insert(flag.to_owned());
        }
    }
    pub fn build(&self) -> Context {
        Context {
            _flags: self.flags.clone(),
            verbose: self.flags.contains(&Flag::Verbose),
        }
    }
}

#[derive(Debug)]
pub struct Context {
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
