use std::{sync::Arc, thread};

use anyhow::{Result, anyhow, bail};
use crossbeam_channel::{Receiver, unbounded};
use skim::{Skim, SkimItemReceiver, SkimItemSender, prelude::SkimOptionsBuilder};

use crate::types::JetItem;

pub fn select(rx_projects: Receiver<JetItem>) -> Result<String> {
    let options = SkimOptionsBuilder::default()
        .preview(Some(String::new()))
        .build()
        .expect("could not build skim options");

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    thread::spawn(move || {
        while let Ok(proj) = rx_projects.recv() {
            tx_item
                .send(Arc::new(proj))
                .expect("could not send project through channel");
        }
    });

    let output = Skim::run_with(&options, Some(rx_item));

    if let Some(output) = output {
        if output.is_abort {
            bail!("cancelled by the user")
        }

        let path = output
            .selected_items
            .first()
            .ok_or(anyhow!("no project selected"))?;

        return Ok(path.output().into_owned());
    }

    bail!("no project selected");
}
