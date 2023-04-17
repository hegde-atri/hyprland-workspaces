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
fn get_workspaces() -> Result<Vec<Workspace>, io::Error> {
    match exec("hyprctl workspaces", command::get_pwd(None).as_path()) {
        Ok(o) => {
            // Parse output to contain only required fields
            let output = String::from_utf8(o.stdout).unwrap();
            let output_list = output.split("workspace ID ").collect::<Vec<_>>();
            let list: Vec<Workspace> = output_list
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    println!("{}", s.trim());
                    // HACK
                    return Workspace::from_string(s).unwrap();
                })
                .collect();
            println!("{:?}", list);
            return Ok(list);
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
