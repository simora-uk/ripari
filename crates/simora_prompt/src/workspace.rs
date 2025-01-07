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

#[cfg(test)]
mod tests {
    use super::*;
    // use std::path::Path;

    #[test]
    fn test_workspace_default() {
        let workspace = Workspace::default();
        assert_eq!(workspace.root(), &PathBuf::from(""));
    }

    #[test]
    fn test_workspace_new() {
        let workspace = Workspace::new();
        assert_eq!(workspace.root(), &PathBuf::from("."));
    }

    #[test]
    fn test_workspace_root_path() {
        let workspace = Workspace::new();
        let root = workspace.root();
        assert!(root.is_relative());
        assert_eq!(root.to_str().unwrap(), ".");
    }

    #[test]
    fn test_workspace_debug_format() {
        let workspace = Workspace::new();
        assert_eq!(format!("{:?}", workspace), "Workspace { root: \".\" }");
    }
}
