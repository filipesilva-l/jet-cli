use std::{
    borrow::Cow,
    env::{self},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    thread::spawn,
};

use anyhow::{Context, Result, anyhow, bail};
use ignore::{WalkBuilder, types::TypesBuilder};
use skim::{
    ItemPreview, Skim, SkimItem, SkimItemReceiver, SkimItemSender,
    prelude::{SkimOptionsBuilder, unbounded},
};

struct Project {
    pub name: String,
    path: PathBuf,
}

impl Project {
    fn new(root: &Path, path: &Path) -> Self {
        let path = path.parent().unwrap().to_owned();
        let name = path
            .strip_prefix(root)
            .expect("path did not have root as it start")
            .to_string_lossy()
            .into_owned();

        Self { name, path }
    }
}

impl SkimItem for Project {
    fn text(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn preview(&self, _context: skim::PreviewContext) -> skim::ItemPreview {
        ItemPreview::Command(format!("ls -1 '{}'", self.path.to_str().unwrap()))
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(self.path.to_str().unwrap())
    }
}

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

    let mut types_builder = TypesBuilder::new();
    types_builder.add("csproj", "*.csproj").unwrap();
    types_builder.add("sln", "*.sln").unwrap();
    types_builder.add("slnx", "*.slnx").unwrap();
    types_builder.add("cargotoml", "Cargo.toml").unwrap();
    types_builder.select("all");

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    spawn(move || {
        for root in roots {
            let types = types_builder
                .build()
                .expect("could not create types for the walkbuilder");

            for entry in WalkBuilder::new(&root)
                .standard_filters(true)
                .types(types)
                .build()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                tx_item
                    .send(Arc::new(Project::new(&root, entry.path())))
                    .unwrap()
            }
        }

        drop(tx_item);
    });

    let options = SkimOptionsBuilder::default()
        .preview(Some(String::new()))
        .build()
        .unwrap();

    let output = Skim::run_with(&options, Some(rx_item));

    if let Some(output) = output {
        if output.is_abort {
            bail!("cancelled by the user")
        }

        let path = output
            .selected_items
            .first()
            .ok_or(anyhow!("no project selected"))?;

        println!("{}", path.output());
    }

    Ok(())
}
