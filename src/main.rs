use std::io;

use args::{Action, HyprWorkspaceArgs};
use clap::Parser;
use workspaces::Workspace;

use crate::command::exec;

mod args;
mod command;
mod workspaces;

fn main() {
    let args = HyprWorkspaceArgs::parse();

    match args.action {
        Action::Windowname => {
            get_active_windowname();
        }
        Action::Workspaces => {
            get_workspaces_json();
        }
        Action::Eww => {
            get_widgets();
        }
    }
}

/// Print out Workspace information parsed from `hyprctl workspaces` with the workspace numbers in json format
fn get_workspaces_json() {
    let _ = get_workspaces();
    println!("Workspaces json");
}

/// Get workspace information parsed from `hyprctl workspaces` with the workspace numbers.
fn get_workspaces() -> Result<String, io::Error> {
    match exec("hyprctl workspaces", command::get_pwd(None).as_path()) {
        Ok(o) => {
            // Parse output to contain only required fields
            let mut output = String::from_utf8(o.stdout).unwrap();
            output = output.split("workspace ID ").collect();
            let output_list = output.split("\n").collect::<Vec<_>>();
            let list: Vec<Workspace> = output_list
                .iter()
                .map(|s| {
                    // keep
                    Workspace {
                        id: 1,
                        status: 1,
                        monitor: 1,
                    }
                })
                .collect();
            return Ok(output);
        }
        Err(err) => Err(err),
    }
}

/// Print out the String containing the windowname
fn get_active_windowname() {
    println!("Windowname");
    todo!();
}

/// Return eww widgets based on workspaces open
fn get_widgets() {
    let _ = get_workspaces();
}
