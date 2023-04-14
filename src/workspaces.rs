use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub status: usize,
    // TODO
    pub monitor: usize,
}

impl Workspace {
    fn from_string(src: &str) -> Result<Workspace, std::io::Error> {
        todo!();
    }
}
