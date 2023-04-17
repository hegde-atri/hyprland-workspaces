use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::command::{self, exec};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    /// Can be numbers 0-3
    /// 0: empty workspace
    /// 1: occupied workspace
    /// 2: active workspace
    pub status: usize,
    // TODO
    pub monitor: usize,
}

impl Workspace {
    pub fn from_string(src: &str) -> Result<Workspace, std::io::Error> {
        Ok(Workspace {
            id: src.split_once(" ").unwrap().0.parse::<usize>().unwrap(),
            status: Workspace::get_status(src).unwrap(),
            monitor: 1,
        })
    }

    fn get_status(src: &str) -> Result<usize, std::io::Error> {
        let id = src.split_once(" ").unwrap().0.parse::<usize>().unwrap();
        let windows = src
            .lines()
            .filter(|s| s.contains("windows:"))
            .map(|s| s.split_once("windows: ").unwrap().1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        println!("{}", windows);
        let active = match exec("hyprctl activewindow", command::get_pwd(None).as_path()) {
            Ok(o) => {
                let output = String::from_utf8(o.stdout).unwrap();
                let aw = output
                    .lines()
                    .filter(|s| s.contains("workspace:"))
                    .map(|s| s.split(" ").nth(1).unwrap())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                Ok(id == aw)
            }
            Err(err) => Err(err),
        };

        if active.unwrap() {
            return Ok(3);
        }
        match windows {
            0 => Ok(0),
            _ => Ok(1),
        }
    }
}
