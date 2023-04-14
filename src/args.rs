use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct HyprWorkspaceArgs {
    /// Describe data to get.
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Returns workspace info in json format
    /// The status of each workspace is such that:
    /// 0 -> Inactive, no windows
    /// 1 -> Inactive, with windows
    /// 2 -> Active
    Workspaces,
    /// Returns active window name as a string.
    Windowname,
    /// Returns my eww widgets for workspaces.
    Eww,
}
