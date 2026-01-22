mod parser;
mod types;
mod client;
mod commands;
mod settings;

use parser::parse_args;
use types::Command;
use settings::Settings;
use commands::init;
use dirs::config_local_dir;
use std::path::PathBuf;

fn main() {
    println!("Welcome to nelson");

    let config_file_path: PathBuf = get_config_file_path();

    let settings: Settings = Settings::load(&config_file_path);

    let args: Vec<String> = std::env::args().skip(1).collect();

    let command: Command = parse_args(args, settings.prompt.default_mode);

    println!("Got command: {:?}", command);

    match command {
        Command::Wtf => return,
        Command::InitCmd => return init(&config_file_path),
        Command::NoCommand => return,
        Command::Prompt(_prompt) => ()
    }
    // Parse console args

    // Determine custom commands (wtf, init)
    //   Delgate if custom to module commands
    // 
    // Have a clean prompt and mode
    // Use module client to send the query (ex: fn prompt(mode: Mode, prompt: Prompt))
    //
    // Print output. If --cmd mode, prompt for copy / execute
    //
    //
} 

fn get_config_file_path() -> PathBuf {
    let mut configs_path: PathBuf = config_local_dir().unwrap();
    configs_path.push("nelson");
    configs_path.push("config.toml");
    return configs_path;
}

