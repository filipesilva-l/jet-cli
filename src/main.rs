use anyhow::*;

mod config;
mod query;
mod selection;
mod types;

fn main() -> Result<()> {
    let config = config::Config::new()?;
    config.validate()?;

    let selected_path = selection::select(query::query_files(config.roots))?;

    println!("{}", selected_path);

    Ok(())
}
