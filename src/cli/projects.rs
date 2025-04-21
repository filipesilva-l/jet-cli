use crate::{config::Config, handlers, query::EntryType};
use anyhow::Result;

use super::Cli;

pub fn run(config: &Config, cli: &Cli) -> Result<()> {
    let rx_jet = crate::query::query_files(&config.roots, |e| matches!(e, EntryType::Project));

    if cli.no_selection {
        handlers::print(rx_jet)
    } else {
        handlers::select(rx_jet)
    }
}
