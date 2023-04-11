use args::HyprWorkspaceArgs;
use clap::Parser;

mod args;

fn main() {
    let args = HyprWorkspaceArgs::parse();
}
