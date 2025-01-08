use crate::diagnostics::CliDiagnostic;
use simora_configuration::{Merge, PartialMarkdownFormatterConfiguration};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Find all configuration files from the current directory up to the root
    /// Returns a list of configurations in order from most specific (current directory)
    /// to least specific (root directory)
    pub fn find_configurations(
        &self,
    ) -> Result<Vec<PartialMarkdownFormatterConfiguration>, CliDiagnostic> {
        let mut configs = Vec::new();
        let mut current_dir = self
            .root
            .canonicalize()
            .map_err(|e| CliDiagnostic::error(format!("Failed to canonicalize path: {}", e)))?;

        loop {
            let config_path = current_dir.join("ripari.json");
            if config_path.exists() {
                let content = fs::read_to_string(&config_path).map_err(|e| {
                    CliDiagnostic::error(format!("Failed to read config file: {}", e))
                })?;

                let config: PartialMarkdownFormatterConfiguration = serde_json::from_str(&content)
                    .map_err(|e| {
                        CliDiagnostic::error(format!("Failed to parse config file: {}", e))
                    })?;

                let is_root = config.root;
                configs.push(config);

                if is_root {
                    break;
                }
            }

            if !current_dir.pop() {
                break;
            }
        }

        Ok(configs)
    }

    /// Load and merge all configurations from the current directory up to the root
    pub fn load_merged_configuration(
        &self,
    ) -> Result<PartialMarkdownFormatterConfiguration, CliDiagnostic> {
        let configs = self.find_configurations()?;
        if configs.is_empty() {
            return Ok(PartialMarkdownFormatterConfiguration::default());
        }

        let mut merged = configs.last().unwrap().clone(); // Start with the most general config
        for config in configs.iter().rev().skip(1) {
            // Skip the last one since we started with it
            merged.merge_with(config.clone());
        }

        Ok(merged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_default() {
        let workspace = Workspace::default();
        assert_eq!(
            workspace.root(),
            &std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        );
    }

    #[test]
    fn test_workspace_new() {
        let workspace = Workspace::new();
        assert_eq!(
            workspace.root(),
            &std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        );
    }

    #[test]
    fn test_workspace_root_path() {
        let workspace = Workspace::new();
        let root = workspace.root();
        assert!(root.is_absolute());
        assert_eq!(
            root,
            &std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        );
    }

    #[test]
    fn test_workspace_debug_format() {
        let workspace = Workspace::new();
        let expected_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        assert_eq!(
            format!("{:?}", workspace),
            format!("Workspace {{ root: {:?} }}", expected_path)
        );
    }
}
