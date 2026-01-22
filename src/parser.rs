use crate::types::Command;
use crate::types::Prompt;
use crate::types::Mode;


pub fn parse_args(args: Vec<String>, default_mode: Mode) -> Command {
    match args.get(0) {
        Some(s) if s == "init" && args.len() == 1 => return Command::InitCmd,
        Some(s) if s == "wtf" && args.len() == 1 => return Command::Wtf,
        Some(_) => Command::Prompt(parse_prompt_args(&args, default_mode)),
        None => return Command::NoCommand
    }
}

fn parse_prompt_args(args: &Vec<String>, default_mode: Mode) -> Prompt {
    let mut prompt_words: Vec<String> = Vec::new();
    let mut extra_flags: Vec<String> = Vec::new();

    let mut mode: Mode = default_mode;

    for arg in args {
        match parse_flag_str(arg) {
            Some(f) => {
                if let Some(m) = Mode::from_flag(f) {
                    mode = m;
                } else {
                    extra_flags.push(f.to_string())
                }
            }
            None => prompt_words.push(arg.to_string())
        }
    }
    Prompt{prompt: prompt_words.join(" ").to_string(), mode: mode, flags: extra_flags}
}

fn parse_flag_str(arg: &String) -> Option<&str> {
    if let Some(flag) = arg.strip_prefix("--") {
        return Some(flag);
    } else if let Some(flag) = arg.strip_prefix("-") {
        return Some(flag);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    // -------------------------
    // parse_flag_str tests
    // -------------------------

    #[test]
    fn parses_long_flag() {
        assert_eq!(parse_flag_str(&"--cmd".to_string()), Some("cmd"));
    }

    #[test]
    fn parses_short_flag() {
        assert_eq!(parse_flag_str(&"-n".to_string()), Some("n"));
    }

    #[test]
    fn ignores_non_flag() {
        assert_eq!(parse_flag_str(&"hello".to_string()), None);
    }

    // -------------------------
    // parse_prompt_args tests
    // -------------------------

    #[test]
    fn builds_prompt_and_uses_default_mode() {
        let prompt = parse_prompt_args(
            &s(&["how", "to", "move", "files"]),
            Mode::Neat,
        );

        assert_eq!(prompt.prompt, "how to move files");
        assert_eq!(prompt.mode, Mode::Neat);
        assert!(prompt.flags.is_empty());
    }

    #[test]
    fn mode_flag_overrides_default() {
        let prompt = parse_prompt_args(
            &s(&["how", "to", "move", "-cmd", "files"]),
            Mode::Neat,
        );

        assert_eq!(prompt.mode, Mode::Cmd);
        assert_eq!(prompt.prompt, "how to move files");
    }

    #[test]
    fn last_mode_flag_wins() {
        let prompt = parse_prompt_args(
            &s(&["-n", "something", "--cmd", "--long"]),
            Mode::Cmd,
        );

        assert_eq!(prompt.mode, Mode::Long);
    }

    #[test]
    fn unknown_flags_are_collected() {
        let prompt = parse_prompt_args(
            &s(&["do", "thing", "--foo", "-bar"]),
            Mode::Neat,
        );

        assert_eq!(prompt.flags, vec!["foo".to_string(), "bar".to_string()]);
    }

    // -------------------------
    // parse_args tests
    // -------------------------

    #[test]
    fn parses_init_command() {
        let cmd = parse_args(s(&["init"]), Mode::Neat);
        assert!(matches!(cmd, Command::InitCmd));
    }

    #[test]
    fn parses_wtf_command() {
        let cmd = parse_args(s(&["wtf"]), Mode::Neat);
        assert!(matches!(cmd, Command::Wtf));
    }

    #[test]
    fn parses_prompt_command() {
        let cmd = parse_args(
            s(&["move", "branches", "--cmd"]),
            Mode::Neat,
        );

        match cmd {
            Command::Prompt(p) => {
                assert_eq!(p.mode, Mode::Cmd);
                assert_eq!(p.prompt, "move branches");
            }
            _ => panic!("Expected Prompt command"),
        }
    }

    #[test]
    fn no_args_returns_no_command() {
        let cmd = parse_args(vec![], Mode::Neat);
        assert!(matches!(cmd, Command::NoCommand));
    }
}

