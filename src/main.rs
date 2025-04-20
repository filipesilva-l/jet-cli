use std::{
    env::{self},
    path::PathBuf,
    str::FromStr,
};

use anyhow::*;

mod query;
mod selection;
mod types;

fn main() -> Result<()> {
    let roots: Vec<_> = env::var("JET_ROOTS")
        .with_context(|| "could not determine JET_ROOTS")?
        .split(',')
        .filter(|val| !val.is_empty())
        .filter_map(|val| PathBuf::from_str(val).ok())
        .map(|path| path.canonicalize().expect("não foi possível canonilizar"))
        .collect();

    if roots.is_empty() {
        return Err(anyhow!("no roots were found"));
    }

    for root in roots.iter() {
        for r2 in roots.iter() {
            if root != r2 && r2.starts_with(root) {
                bail!("root {root:?} and {r2:?} colide");
            }
        }
    }

    let rx_item = query::query_files(roots);

    let selected_path = selection::select(rx_item)?;

    println!("{}", selected_path);

    Ok(())
}
