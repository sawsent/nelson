use crate::context::Context;
use crate::errors::NelsonError;
use crate::domain::Mode;

pub trait Backend {
    fn query(&self, prompt: &str, mode: &Mode, ctx: &Context) -> Result<String, NelsonError>;
}

pub mod ollama;

