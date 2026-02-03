[![Crates.io](https://img.shields.io/crates/v/nelson)](https://crates.io/crates/nelson)
[![Crates.io](https://img.shields.io/crates/d/nelson)](https://crates.io/crates/nelson)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sawsent/nelson/rust.yml?branch=main)](https://github.com/sawsent/nelson/actions/workflows/rust.yml?query=branch%3Amain)
[![Coverage Status](https://codecov.io/gh/sawsent/nelson/branch/main/graph/badge.svg)](https://app.codecov.io/github/sawsent/nelson)

# nelson

nelson is a local-first CLI AI assistant that generates shell commands, explains errors, and answers programming questions using a self-hosted LLM or an openai model.

It currently supports the following backends:
* ollama
* openai

---

## Instalation
Install via cargo
```bash
cargo install nelson
```
Or build from source:
```bash
git clone https://github.com/sawsent/nelson
cd nelson
cargo build --release
```

---

## Usage

nelson has **one main query command** and **one special command**.

Most of the time, you just ask nelson a question. The way it answers is controlled by **modes**, which can be switched using flags placed **anywhere** in the prompt.

If multiple mode flags are provided, **the last one wins**.

---

## Modes

- `--cmd` Generate a shell command
- `-n` Neat (concise answer)
- `-l` Long (detailed explanation)
- `--code` Code-only output

A default mode is used if no flag is provided.

### Examples

```bash
nelson how can I move files -l in unix
nelson --code implement binary search in python
nelson move current branch to new-branch --cmd
```

---

## Configuration

nelson reads configuration from a file located at:

- **Linux:** `~/.config/nelson/config.toml`
- **macOS:** `~/Library/Application Support/nelson/config.toml`
- **Windows:** `%APPDATA%\nelson\config.toml`

Example:

```toml
[backend]
provider = "ollama"
url = "http://localhost:11434/api/chat"

[llm]
model = "llama3.2"

[nelson]
default_mode = "cmd"
```

### Initialize the config file

Run:

```bash
nelson init
```

This creates the default config file (if it doesn't exist) and prints the file path so you can edit it.

---

## Roadmap

- **v0.1.0**: Out of the box experience
    - [ ] wtf command
    - [ ] coverage

