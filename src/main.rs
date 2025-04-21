use anyhow::*;
use clap::Parser;
use cli::Cli;

mod cli;
mod config;
mod handlers;
mod query;
mod types;

fn main() -> Result<()> {
    human_panic::setup_panic!();

    let config = config::Config::new()?;
    config.validate()?;

    let cli = Cli::parse();
    cli.run(&config)
}
