use std::fs;
use std::path::PathBuf;

use crate::context::Context;
use crate::settings::{Settings};
use crate::utils::save_settings;
use crate::domain::{Init, Prompt};
use crate::backend::Backend;

pub fn prompt<T: Backend>(cmd: &Prompt, backend: &T, ctx: &Context) {

    let result = backend.query(&cmd.prompt, &cmd.mode, ctx);

    match result {
        Ok(resp) => println!("{}", resp),
        Err(nerr) => println!("{}", nerr)
    }

}

pub fn init(_cmd: &Init, fp: &PathBuf, ctx: &Context) {
    println!("initializing nelson config at {:?}", fp);

    if let Some(parent) = fp.parent() {
        if !parent.exists() {
            ctx.vprint(format_args!(
                "creating config directory {:?}",
                parent
            ));

            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!(
                    "nelson: failed to create config directory {:?}: {}",
                    parent, e
                );
                return;
            }
        }
    }

    if fp.exists() {
        println!("config file already exists, not overwriting");
        return;
    }

    let settings = Settings::default();

    ctx.vprint(format_args!("writing default config file"));

    if let Err(e) = save_settings(&settings, fp, ctx) {
        eprintln!(
            "nelson: failed to write config file {:?}: {}",
            fp, e
        );
    } 
}

