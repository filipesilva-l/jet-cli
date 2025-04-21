use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::config::Config;

pub fn run() -> Result<()> {
    if let Ok(value) = std::env::var("JET_SHELL") {
        if value == "true" {
            bail!("edit cannot be invoked from the shell alias");
        }
    }

    let editor = std::env::var("EDITOR").with_context(|| "could not read the $EDITOR variable")?;

    let path = Config::get_config_file_path()?;

    Command::new(editor)
        .arg(&path)
        .status()
        .with_context(|| "something went wrong")?;

    Ok(())
}
