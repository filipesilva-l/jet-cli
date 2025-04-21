use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use skim::{ItemPreview, SkimItem};

pub struct JetItem {
    pub name: String,
    pub path: PathBuf,
}

impl JetItem {
    pub fn new(root: &Path, path: &Path) -> Self {
        let path = if path.is_file() {
            path.parent().unwrap()
        } else {
            path
        }
        .to_owned();

        let name = path
            .strip_prefix(root)
            .expect("path did not have root as it start")
            .to_string_lossy()
            .into_owned();

        Self { name, path }
    }
}

impl SkimItem for JetItem {
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
