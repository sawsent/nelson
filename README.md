
# nelson

nelson is a local-first CLI AI assistant that generates shell commands, explains errors, and answers programming questions using a self-hosted LLM.

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

## Special command

### `nelson wtf`

Explain the last executed command error.

```bash
nelson wtf
```

```
The error you're seeing usually happens due to connectivity issues with the database.
Consider trying:
1. ...
2. ...
3. ...
```

---

## Configuration

nelson reads configuration from a file located at:

- **Linux:** `~/.config/nelson/config.toml`
- **macOS:** `~/Library/Application Support/nelson/config.toml`
- **Windows:** `%APPDATA%\nelson\config.toml`

Example:

```toml
[llm]
host = "localhost"
port = 11434

[mode]
default = "cmd"
```

### Initialize the config file

Run:

```bash
nelson init
```

This creates the default config file (if it doesn't exist) and prints the file path so you can edit it.

---

## Roadmap

- **v1**: Out of the box experience
    - [ ] interactive setup
    - [ ] built-in server management
    - [ ] automated LLM download and setup
- **v2**: Customization (tentative)
    - [ ] multiple LLM backends
    - [ ] generalized nelson backend (create your own custom adapter to any local or hosted llm)

