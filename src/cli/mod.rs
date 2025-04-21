use clap::{Parser, Subcommand};

use crate::config::Config;

mod edit;
mod projects;
mod repos;
mod smart;
mod up;

#[derive(Subcommand, Clone, Debug)]
#[command(author, version, about, long_about = None, verbatim_doc_comment)]
#[command(propagate_version = true)]
pub enum Command {
    /// DEFAULT - Context-aware search that adapts based on current location:
    /// - When inside a repository: Searches for projects within the current repository
    /// - When outside any repository: Searches for repositories within configured root directories
    Smart,

    /// Repository-focused search that finds only repositories
    /// within the configured root directories, ignoring individual projects
    Repos,

    /// Project-focused search that finds only projects
    /// within the configured root directories, regardless of repository structure
    Projects,

    /// Jumps to the root of the repository or, if not inside a repository, to the home directory
    Up,

    /// Open's the config using the $EDITOR
    Edit,
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Disables the native fuzzy finder selection
    #[arg(short = 'n', long = "no_selection", global = true)]
    no_selection: bool,
}

impl Cli {
    pub fn run(&self, config: &Config) -> anyhow::Result<()> {
        match self.command {
            Some(Command::Smart) | None => smart::run(config, self),
            Some(Command::Repos) => repos::run(config, self),
            Some(Command::Projects) => projects::run(config, self),
            Some(Command::Up) => up::run(),
            Some(Command::Edit) => edit::run(),
        }
    }
}
