use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use arboard::Clipboard;

use crate::context::Context;
use crate::domain::{Init, Prompt, Mode};
use crate::backend::Backend;
use crate::printer;

pub fn prompt(cmd: &Prompt, backend: &Box<dyn Backend>, ctx: &Context) {

    let result = backend.query(&cmd.prompt, &cmd.mode, ctx);

    match result {
        Ok(resp) => {
            printer::print_with_bat(&resp);

            match cmd.mode {
                Mode::Cmd => prompt_copy(&resp, ctx),
                Mode::Code => prompt_copy(&remove_markdown_block(&resp), ctx),
                _ => ()
            }
        }
        Err(nerr) => println!("{}", nerr)
    }
}

fn remove_markdown_block(s: &str) -> String {
    let mut out = Vec::new();

    for line in s.lines() {
        if !line.trim().starts_with("```") {
            out.push(line);
        }
    }

    out.join("\n")
}

fn prompt_copy(resp: &str, ctx: &Context) {
    print!("[Enter] copy | [Other] quit >> ");
    let _ = io::stdout().flush();
    match read_line() {
        Ok(l) if l.trim().is_empty() => {
            match copy_to_clipboard(&resp) {
                Ok(_) => println!("Copied"),
                Err(err) => {
                    println!("There was an error copying the response.");
                    ctx.vprint(format_args!("{:?}", err));
                }
            }
        }
        Ok(_) => println!("Not copied"),
        Err(e) => {
            ctx.vprint(format_args!("There was an error reading stdin: {:?}", e));
            println!("Not copied")
        }
    }

}

fn copy_to_clipboard(text: &str) -> Result<(), arboard::Error> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}

fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
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

    ctx.vprint(format_args!("writing default config file"));

    if let Err(e) = fs::write(fp, crate::r#static::example_config::CONFIG_STRING) {
        eprintln!(
            "nelson: failed to write config file {:?}: {}",
            fp, e
        );
    } 
}

