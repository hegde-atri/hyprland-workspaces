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
    pub class: String,
    // TODO
    pub monitor: usize,
}

impl Workspace {
    pub fn from_string(src: &str) -> Result<Workspace, std::io::Error> {
        Ok(Workspace {
            id: src.split_once(" ").unwrap().0.parse::<usize>().unwrap(),
            status: Workspace::get_status(src).unwrap(),
            class: match Workspace::get_status(src).unwrap() {
                1 => "empty-workspace".to_string(),
                2 => "occupied-workspace".to_string(),
                3 => "active-workspace".to_string(),
                _ => "".to_string(),
            },
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
        let active = match exec("hyprctl monitors", command::get_pwd(None).as_path()) {
            Ok(o) => {
                let output = String::from_utf8(o.stdout).unwrap();
                let aw = output
                    .lines()
                    .filter(|s| s.contains("active workspace:"))
                    .map(|s| s.split(" ").nth(2).unwrap())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap_or(1);
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
