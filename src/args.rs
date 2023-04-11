use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct HyprWorkspaceArgs {
    /// The directory to search for git repositories (optional)
    /// Will return summarised git status for each repo.
    pub dir: Option<String>,
}
