use crate::context::Context;
use crate::errors::NelsonError;

pub trait Backend {
    fn get_type() -> &'static str;
    fn query(&self, system_prompt: &str, user_prompt: &str, model_name: &str, ctx: &Context) -> Result<String, NelsonError>;
}

pub mod ollama;

