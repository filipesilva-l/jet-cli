use std::{path::PathBuf, thread};

use crossbeam_channel::{Receiver, Sender};
use ignore::{WalkBuilder, types::TypesBuilder};

use crate::types::Project;

pub fn query_files(roots: Vec<PathBuf>) -> Receiver<Project> {
    let mut types_builder = TypesBuilder::new();
    types_builder.add("csproj", "*.csproj").unwrap();
    types_builder.add("sln", "*.sln").unwrap();
    types_builder.add("slnx", "*.slnx").unwrap();
    types_builder.add("cargotoml", "Cargo.toml").unwrap();
    types_builder.select("all");

    let (tx_item, rx_item): (Sender<Project>, Receiver<Project>) = crossbeam_channel::unbounded();

    thread::spawn(move || {
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
                tx_item.send(Project::new(&root, entry.path())).unwrap()
            }
        }

        drop(tx_item);
    });

    rx_item
}
