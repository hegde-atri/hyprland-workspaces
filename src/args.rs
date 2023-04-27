use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct HyprWorkspaceArgs {
    /// Describe data to get.
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Returns workspace info in json format {id, status, monitor}.
    ///
    /// id is the workspace number.
    ///
    /// The status of each workspace is such that:
    /// 0 -> empty,
    /// 1 -> has windows,
    /// 2 -> active workspace.
    ///
    /// Monitors is currently not supported and will always return 1.
    Workspaces,
    /// Returns active window name as a string.
    Windowname,
    /// Does not work.
    Eww,
}
