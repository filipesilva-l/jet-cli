use std::sync::Arc;

use crate::{config::Config, handlers, query::EntryType};
use anyhow::Result;

use super::{Cli, repos, up};

pub fn run(config: &Config, cli: &Cli) -> Result<()> {
    let repo_path = up::get_repo_path()?;

    if let Some(repo_path) = repo_path {
        let rx_jet = crate::query::query_files(&Arc::new(vec![repo_path]), |e| {
            matches!(e, EntryType::Project)
        });

        if cli.no_selection {
            return handlers::print(rx_jet);
        } else {
            return handlers::select(rx_jet);
        }
    }

    repos::run(config, cli)
}
