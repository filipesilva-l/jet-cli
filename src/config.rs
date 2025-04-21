use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result, anyhow, bail};
use dirs::config_dir;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub roots: Arc<Vec<PathBuf>>,
}

impl Config {
    pub fn new() -> Result<Self> {
        Self::try_from_file().with_context(|| "could not parse config from either file or env")
    }

    fn try_from_file() -> Result<Self> {
        let mut config_path = config_dir().ok_or(anyhow!("could not determine the config path"))?;
        config_path.push("jet-cli/config.toml");

        if !std::fs::exists(&config_path).unwrap_or(false) {
            bail!("config file {:?} not found", &config_path);
        }

        let content = std::fs::read_to_string(&config_path)?;

        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.roots.is_empty() {
            return Err(anyhow!("no roots were found"));
        }

        for root in self.roots.iter() {
            for r2 in self.roots.iter() {
                if root != r2 && r2.starts_with(root) {
                    bail!("root {root:?} and {r2:?} colide");
                }
            }
        }

        Ok(())
    }
}
