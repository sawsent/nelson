use crate::context::Context;
use dirs::config_local_dir;
use std::path::PathBuf;

pub fn get_config_file_path(ctx: &Context) -> PathBuf {
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
