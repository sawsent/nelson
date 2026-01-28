pub const CONFIG_STRING: &'static str =
r#"[backend]
provider = "ollama"
url = "http://localhost:11434/api/chat"
# Uncomment to use authorization  
#token = "token"

# fallback urls to use when provider is changed on-the-fly with --provider=<provider> --model=<model>
fallback_urls = {
    openai = "https://api.openai.com/v1/chat/completions",
    ollama = "http://localhost:11434/api/chat"
}

# Example OpenAi backend config
# provider = "openai"
# url = "https://api.openai.com/v1/chat/completions"
# token = "token"

[llm]
model = "llama3.2"

[nelson]
# ["cmd", "neat", "long", "code"]
default_mode = "cmd"
"#;
