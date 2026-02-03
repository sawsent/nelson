pub const GENERAL: &str = r#"nelson — local-first CLI AI assistant

USAGE:
    nelson [FLAGS] <query>
    nelson init

DESCRIPTION:
    Ask nelson a question and it responds using your local LLM.
    Output style is controlled by mode flags.
    Flags may appear anywhere in the prompt.
    If multiple modes are provided, the last one wins.

MODES:
    --cmd       output ONLY the shell command
    -n          neat (concise, readable answer)
    -l          long (detailed explanation)
    --code      output ONLY a single code block

SPECIAL COMMANDS:
    init        create default config file

EXAMPLES:
    nelson how can I move files -l in unix
    nelson --code implement binary search in python
    nelson move current branch to new-branch --cmd
    nelson init

CONFIG:
    Linux    ~/.config/nelson/config.toml
    macOS    ~/Library/Application Support/nelson/config.toml
    Windows  %APPDATA%\nelson/config.toml

NOTES:
    - default mode is used when no flag is provided
"#;

pub const VERSION: &str = "nelson version 0.0.1";
