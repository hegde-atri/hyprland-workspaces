/// Temporary class whilst I sort out my utils class
///
use std::{
    io::{Error, ErrorKind, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use which::which;

/// Returns output to give command
///
/// It takes in the command/binary to execute, optional flags and the path
/// to execute it in.
///
/// It will only execute if the provided `cmd` is found on host and `cwd` exists
/// as a directory.
///
/// # Arguments
///
/// * `cmd` - The binary to execute.
/// * `cwd` - The directory to execute in.
///
/// # Examples
/// ```
/// exec("hyprctl workspaces", Path::new("/home/user/repo"))
/// ```
pub fn exec(cmd: &str, cwd: &Path) -> Result<Output, Error> {
    // Check if binary for command exists
    match which(cmd.split(' ').next().unwrap()) {
        Ok(_) => (),
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not find specified command: {}", cmd),
            ))
        }
    }
    // Check if path exists
    if !cwd.exists() {
        return Err(Error::new(ErrorKind::Other, "Specified path is invalid!"));
        // path is not a directory
        if !cwd.is_dir() {
            return Err(Error::new(
                ErrorKind::Other,
                "Specified path is not a directory",
            ));
        }
    }
    // Now execute the command
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .current_dir(&cwd.as_os_str())
            .args(["/C", cmd])
            .output();
    } else {
        return Command::new("sh")
            .current_dir(&cwd.as_os_str())
            .arg("-c")
            .arg(cmd)
            .output();
    };
}

/// Returns a Pathbuf of current working dir or the dir if provided.
pub fn get_pwd(dir: Option<&Path>) -> PathBuf {
    let pwd = match std::env::current_dir() {
        Ok(v) => PathBuf::from(v),
        Err(err) => panic!("Couldn't find current dir: {}", err),
    };

    return match dir {
        Some(v) => v.to_path_buf(),
        None => pwd,
    };
}
