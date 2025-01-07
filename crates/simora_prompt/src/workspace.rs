use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            root: PathBuf::from("."),
        }
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }
}
