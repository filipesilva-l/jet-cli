use std::path::{Path, PathBuf};

use crate::query::{EntryType, categorize};
use anyhow::{Result, anyhow};

pub fn run() -> Result<()> {
    let repo_path = get_repo_path()?;

    if let Some(repo_path) = repo_path {
        println!("{}", repo_path.to_str().unwrap());

        return Ok(());
    }

    let home_dir = dirs::home_dir().ok_or(anyhow!("could not determine home dir"))?;

    println!("{}", home_dir.to_str().unwrap());

    Ok(())
}

pub fn get_repo_path() -> Result<Option<PathBuf>> {
    let current_dir = std::env::current_dir()?;

    let mut current_dir: Option<&Path> = Some(&current_dir);

    let home_dir = dirs::home_dir().ok_or(anyhow!("could not determine home dir"))?;

    while let Some(dir) = current_dir {
        if dir == home_dir {
            break;
        }

        for entry in std::fs::read_dir(dir)?.filter_map(|e| e.ok()) {
            let path = entry.path();

            if matches!(categorize(entry), EntryType::Repository) {
                let root = path.parent().unwrap();

                return Ok(Some(root.to_owned()));
            }
        }

        current_dir = dir.parent();
    }

    Ok(None)
}
