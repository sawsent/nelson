mod parser;
mod types;
mod client;
mod commands;

use parser::parse_args;
use types::Command;
use types::Mode;

fn main() {
    println!("Welcome to nelson");

    let args: Vec<String> = std::env::args().skip(1).collect();

    let command: Command = parse_args(args, Mode::Cmd);

    println!("Got command: {:?}", command);

    match command {
        Command::Wtf => return,
        Command::InitCmd => return,
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

