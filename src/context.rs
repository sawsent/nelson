use std::collections::HashSet;
use std::fmt::Arguments;

pub struct ContextBuilder {
    pub flags: HashSet<String>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
        }
    }
    pub fn add_flags(&mut self, flags: &HashSet<String>) {
        for flag in flags {
            self.flags.insert(flag.to_string());
        }
    }
    pub fn build(&self) -> Context {
        Context {
            _flags: self.flags.clone(),
            verbose: self.flags.contains(&"verbose".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    _flags: HashSet<String>,
    pub verbose: bool,
}

impl Context {
    pub fn vprint(&self, args: Arguments) {
        if self.verbose {
            eprintln!("[verbose] {}", args);
        }
    }
}
