trait Backend {
    fn query(&self, system_prompt: &str, user_prompt: &str) -> Result<(), ()>;
}

pub mod ollama;
