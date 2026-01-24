use crate::context::Context;
use dirs::config_local_dir;
use std::path::PathBuf;
use std::fs;
use crate::errors::NelsonError;
use crate::settings::Settings;

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

pub fn load_settings(fp: &PathBuf, default: Settings, ctx: &Context) -> Settings {
    match fs::read_to_string(fp) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_else(|err| {
            ctx.vprint(format_args!(
                "nelson: failed to parse config at {:?}. Error: {}. Using default.",
                fp, err
            ));
            default
        }),
        Err(e) => {
            ctx.vprint(format_args!(
                "Could not access the file at {:?}. Error: {:?}. Using default.",
                fp, e
            ));
            default
        }
    }
}

pub fn save_settings(settings: &Settings, fp: &PathBuf, ctx: &Context) -> Result<(), NelsonError> {
    ctx.vprint(format_args!(
        "saving config to {:?}",
        fp
    ));

    let toml = toml::to_string_pretty(settings)
        .map_err(|e| {
            NelsonError::Internal(format!("failed to serialize config: {}", e))
        })?;

    fs::write(fp, toml)
        .map_err(|e| {
            NelsonError::Internal(format!(
                "failed to write config file {:?}: {}",
                fp, e
            ))
        })?;

    ctx.vprint(format_args!(
        "config successfully written"
    ));

    Ok(())
}

