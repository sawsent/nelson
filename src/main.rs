mod backend;
mod context;
mod dispatch;
mod domain;
mod errors;
mod parser;
mod printer;
mod settings;
mod r#static;
mod utils;

use context::{Context, ContextBuilder};
use domain::Command;
use settings::Settings;
use std::path::PathBuf;

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

    let config_file_path = utils::get_config_file_path(&ctx);
    let default_settings: Settings = Settings::default();
    let settings: Settings =
        utils::load_settings(&config_file_path, default_settings.clone(), &ctx);
    ctx.vprint(format_args!("Using settings: {:?}", settings));

    let backend =
        backend::backend_from_settings(&settings.backend, &settings.llm).unwrap_or_else(|err| {
            eprintln!(
                "Unable to load backend {}. Use --verbose for full error. Defaulting to {}.",
                err, default_settings.backend.provider
            );
            ctx.vprint(format_args!("{}", err));
            backend::default_backend()
        });

    let command: Command = parser::parse_args(&args, &settings.nelson.default_mode);
    ctx.vprint(format_args!("Got command: {:?}", command));

    if ctx.is_help {
        help(&ctx, &command, &settings, &config_file_path);
        return;
    }

    if ctx.is_version {
        version();
        return;
    }

    match command {
        Command::WtfCmd(_wtf) => (),
        Command::InitCmd(init) => dispatch::init(&init, &config_file_path, &ctx),
        Command::Prompt(prompt) => dispatch::prompt(&prompt, &*backend, &ctx),
        Command::NoCmd => help(&ctx, &command, &settings, &config_file_path),
    }
}

fn help(_ctx: &Context, _command: &Command, _settings: &Settings, _config_file_path: &PathBuf) {
    println!("{}", r#static::help::GENERAL);
}

fn version() {
    println!("{}", r#static::help::VERSION);
}
