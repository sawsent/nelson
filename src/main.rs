mod client;
mod commands;
mod context;
mod parser;
mod settings;
mod types;

use commands::init;
use context::{Context, ContextBuilder};
use dirs::config_local_dir;
use settings::Settings;
use std::path::PathBuf;
use types::Command;

fn main() {
    let mut context_builder = ContextBuilder::new();

    let args: &Vec<String> = &std::env::args().skip(1).collect();

    context_builder.add_flags(&parser::parse_flags(args));
    let ctx = context_builder.build();
    ctx.vprint(format_args!("verbose mode is ON"));
    ctx.vprint(format_args!("Built context: {:?}", ctx));

    let config_file_path = get_config_file_path(&ctx);

    let settings: Settings = Settings::load(&config_file_path, &ctx);
    ctx.vprint(format_args!("Using settings: {:?}", settings));

    let command: Command = parser::parse(args, settings.prompt.default_mode);
    ctx.vprint(format_args!("Got command: {:?}", command));

    match command {
        Command::Wtf => return,
        Command::InitCmd => return init(&config_file_path, &ctx),
        Command::NoCommand => return,
        Command::Prompt(_prompt) => (),
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
            p
        })
        .map(|mut p| {
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
