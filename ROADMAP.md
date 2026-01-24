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
- [ ] Finalize config structure and defaults
- [ ] Ensure config loads correctly from local path
- [ ] Handle missing config gracefully (suggest running `nelson init`)

### 2. Prompt Command Handler
- [ ] Connect parsed prompt → query backend → output response
- [ ] Ensure prompt flags override default mode (last flag wins)
- [ ] Support default mode from config

### 3. Error Handling
- [ ] Add user-friendly errors:
  - backend unreachable
  - invalid config
  - model not found
  - internal errors
- [ ] Print helpful guidance in CLI

### 4. Tests
- [ ] Unit tests for parsing
- [ ] Unit tests for config loading
- [ ] Integration test for prompt flow (mocked backend)

### 5. README & Documentation
- [ ] Update README to match v0 behavior
- [ ] Include minimal usage examples

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

