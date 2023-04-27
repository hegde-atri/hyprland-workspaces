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
            println!("{}", get_active_windowname().unwrap());
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
    println!(
        "{}",
        serde_json::to_string(&get_workspaces().unwrap()).unwrap()
    );
}

/// Get workspace information parsed from `hyprctl workspaces` with the workspace numbers.
fn get_workspaces() -> Result<Vec<Workspace>, io::Error> {
    match exec("hyprctl workspaces", command::get_pwd(None).as_path()) {
        Ok(o) => {
            // Parse output to contain only required fields
            let output = String::from_utf8(o.stdout).unwrap();
            let output_list = output.split("workspace ID ").collect::<Vec<_>>();
            let mut list: Vec<Workspace> = output_list
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    // HACK
                    return Workspace::from_string(s).unwrap();
                })
                .collect();
            // Sort by id
            list.sort_by(|a, b| a.id.cmp(&b.id));
            return Ok(list);
        }
        Err(err) => Err(err),
    }
}

/// Print out the String containing the windowname
fn get_active_windowname() -> Result<String, io::Error> {
    match exec("hyprctl activewindow", command::get_pwd(None).as_path()) {
        Ok(o) => {
            let output = String::from_utf8(o.stdout).unwrap();

            let name = output
                .split(" ")
                .nth(3)
                .unwrap()
                .split("\n")
                .nth(0)
                .unwrap();
            return Ok(name[0..name.len() - 1].to_string());
        }
        Err(err) => Err(err),
    }
}

/// Return eww widgets based on workspaces open
/// Does not work
fn get_widgets() {
    let workspaces = get_workspaces().unwrap();
    let mut widget = String::new();
    const START: &str = "\n(box :space-evenly true \n";
    const END: &str = "\n)";

    widget.push_str(START);

    // Make the workspace box widgets
    for (i, w) in workspaces.iter().enumerate() {
        widget.push_str("(box :class \"workspace-entry");
        match w.status.to_string().as_str() {
            "1" => widget.push_str(" occupied"),
            "2" => widget.push_str(" empty"),
            "3" => widget.push_str(" current"),
            _ => (),
        }
        widget.push_str("\"");
        widget.push_str(" (label :text \"");
        widget.push_str(w.id.to_string().as_str());
        widget.push_str("\")");
        widget.push_str(")");
        if i != workspaces.len() - 1 {
            widget.push_str("\n");
        }
    }
    widget.push_str(END);
    println!("{}", widget);
}
