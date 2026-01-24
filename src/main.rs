mod client;
mod commands;
mod context;
mod parser;
mod settings;
mod types;
mod dto;

use commands::init;
use context::{Context, ContextBuilder};
use dirs::config_local_dir;
use settings::Settings;
use std::path::PathBuf;
use types::Command;

fn main() {
    let mut context_builder = ContextBuilder::new();

    let full_args: Vec<String> = std::env::args().skip(1).collect();
    let full_cmd = "nelson ".to_string() + &full_args.join(" ");
    context_builder.set_cmd(&full_cmd);

    let (args, flags) = parser::split_args_and_flags(&full_args);
    context_builder.add_flags(&flags);

    let ctx = context_builder.build();
    ctx.vprint(format_args!("verbose mode is ON"));
    ctx.vprint(format_args!("Built context: {:?}", ctx));

    let config_file_path = get_config_file_path(&ctx);


    let settings: Settings = settings::load(&config_file_path, &ctx);
    ctx.vprint(format_args!("Using settings: {:?}", settings));

    let command: Command = parser::parse_args(&args, &settings.prompt.default_mode);
    ctx.vprint(format_args!("Got command: {:?}", command));

    match command {
        Command::Wtf => return,
        Command::InitCmd => return init(&config_file_path, &ctx),
        Command::NoCommand => return,
        Command::Prompt(prompt) => {
            let response = client::query_prompt(&prompt, &settings, &ctx);
            println!("{:#?}", response);
        }
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

fn get_config_file_path(ctx: &Context) -> PathBuf {
    config_local_dir()
        .map(|mut p| {
            p.push("nelson");
            p.push("config.toml");
            p
        })
        .unwrap_or_else(|| {
            ctx.vprint(format_args!(
                "Couldn't get the config file path, using ./nelson.config.toml"
            ));
            PathBuf::from("./nelson.config.toml")
        })
}
