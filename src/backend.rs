trait Backend {
    fn query(&self, system_prompt: &str, user_prompt: &str) -> Result<(), ()>;
}

struct OllamaBackend {
    host: String,
    port: u16
}

impl Backend for OllamaBackend {
    fn query(&self, system_prompt: &str, user_prompt: &str) -> Result<(), ()> {
        Ok(())
    }
}
