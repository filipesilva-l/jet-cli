use std::{fs::DirEntry, path::PathBuf, sync::Arc, thread};

use crossbeam_channel::{Receiver, Sender};
use ignore::{WalkBuilder, WalkState};

use crate::types::JetItem;

pub fn query_files<F>(roots: &Arc<Vec<PathBuf>>, entry_predicate: F) -> Receiver<JetItem>
where
    F: Fn(EntryType) -> bool + Send + Sync + 'static,
{
    let (tx_item, rx_item): (Sender<JetItem>, Receiver<JetItem>) = crossbeam_channel::unbounded();

    let roots = roots.clone();

    thread::spawn(move || {
        for root in roots.as_ref() {
            WalkBuilder::new(root)
                .standard_filters(true)
                .filter_entry(|e| e.path().is_dir())
                .build_parallel()
                .run(|| {
                    Box::new(|entry| {
                        if let Ok(e) = entry {
                            let path = e.path();

                            let entries = std::fs::read_dir(path).unwrap();

                            let found = entries
                                .filter_map(|e| e.ok())
                                .any(|e| entry_predicate(categorize(e)));

                            if found {
                                tx_item.send(JetItem::new(root, path)).unwrap();

                                return WalkState::Skip;
                            }
                        }

                        WalkState::Continue
                    })
                })
        }

        drop(tx_item);
    });

    rx_item
}

pub enum EntryType {
    Unrecognized,
    Project,
    Repository,
}

pub fn categorize(entry: DirEntry) -> EntryType {
    let path = entry.path();

    if let Some(filename) = path.file_name() {
        if filename == "Cargo.toml" || filename == "go.mod" {
            return EntryType::Project;
        }

        if filename == ".git" {
            return EntryType::Repository;
        }
    }

    if let Some(extension) = path.extension() {
        if extension == "csproj" {
            return EntryType::Project;
        }
    }

    EntryType::Unrecognized
}
