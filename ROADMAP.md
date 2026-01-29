# Nelson v0 Roadmap (Remaining Tasks)

This is a minimal checklist for **only what’s left to complete v0**.

---

## ✅ Completed
- Basic CLI parsing
- Mode flags work anywhere in the prompt
- `nelson init` command exists (writes default config)
- `stream=false` set in payload
- Basic output and verbose logic exists

---

## ⬜ Remaining v0 Tasks

### 1. Settings & Config
- [x] Finalize config structure and defaults
- [x] Ensure config loads correctly from local path
- [x] Handle missing config gracefully (suggest running `nelson init`)

### 2. Prompt Command Handler
- [x] Connect parsed prompt → query backend → output response
- [x] Ensure prompt flags override default mode (last flag wins)
- [x] Support default mode from config

### 3. Error Handling
- [x] Add user-friendly errors:
  - backend unreachable
  - invalid config
  - model not found
  - internal errors
- [x] Print helpful guidance in CLI

### 4. OpenAI backend
Local models can be pretty bad. Implementing an open-ai backend will help me.
- [x] OpenAI backend works
- [x] Token in config is loaded correctly

### 4. System prompts
- [ ] Define system prompt and test
    - [ ] Code
    - [ ] Neat
    - [ ] Long
    - [ ] Cmd

### 5. Finalize UX
- [x] Pretty print LLM response
- [x] Prompt for copy
- [ ] Helpful help text

### 5. Tests
- [ ] Unit tests

### 6. README & Documentation
- [ ] Update README to match v0 behavior
- [ ] Test and suggest a model to use 
- [ ] Include minimal usage examples
- [ ] Create a release with all binaries (windows, macos, linux)

---

## v0 Done Checklist
- [ ] `nelson <prompt>` works
- [ ] `nelson --cmd <prompt>` works
- [ ] `nelson --neat <prompt>` works
- [ ] `nelson --long <prompt>` works
- [ ] `nelson --code <prompt>` works
- [ ] `nelson init` works
- [ ] Config loading works
- [ ] Ollama query works end-to-end
- [ ] Tests pass

