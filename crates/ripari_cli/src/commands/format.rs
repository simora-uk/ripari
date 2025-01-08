use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::console::Console;
use crate::diagnostics::CliDiagnostic;
use crate::workspace::Workspace;
use simora_configuration::{
    HeadingsConfig, HorizontalRulesConfig, MarkdownFormatterConfig, PartialFilesConfiguration,
    PartialMarkdownFormatterConfiguration, PartialVcsConfiguration, PunctuationConfig, RulesConfig,
    SmartQuotesConfig,
};
use simora_formatter::{Formatter, MarkdownFormatter};
use simora_glob::Glob;

pub struct FormatCommandPayload {
    pub markdown_formatter: Option<MarkdownFormatterConfig>,
    pub files_configuration: Option<PartialFilesConfiguration>,
    pub vcs_configuration: Option<PartialVcsConfiguration>,
    pub stdin_file_path: Option<String>,
    pub write: bool,
    pub fix: bool,
    pub paths: Vec<OsString>,
    pub staged: bool,
    pub changed: bool,
    pub since: Option<String>,
}

/// Trait for commands that can load editor configuration
pub trait LoadEditorConfig {
    fn load_editor_config(&self) -> Result<PartialMarkdownFormatterConfiguration, CliDiagnostic>;
}

/// Trait for executable commands
pub trait CommandRunner {
    fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic>;
}

#[derive(Debug, Clone, Default)]
pub struct FormatCommand {
    pub write: bool,
    pub fix: bool,
    pub paths: Vec<OsString>,
    pub stdin_file_path: Option<String>,
    pub show_help: bool,
    pub formatter_config: PartialMarkdownFormatterConfiguration,
    pub staged: bool,
    pub changed: bool,
    pub since: Option<String>,
}

impl LoadEditorConfig for FormatCommand {
    fn load_editor_config(&self) -> Result<PartialMarkdownFormatterConfiguration, CliDiagnostic> {
        // Load configuration from workspace
        let workspace = Workspace::new();
        workspace.load_merged_configuration()
    }
}

impl CommandRunner for FormatCommand {
    fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        if self.show_help {
            Self::print_help(console);
            return Ok(());
        }

        // Load configuration
        let config = self.load_editor_config()?;

        console.log(&format!(
            "Formatting files in workspace: {:?}",
            workspace.root()
        ));

        // Create formatter
        let mut formatter = MarkdownFormatter::new();
        formatter
            .apply_configuration(&config)
            .map_err(|e| CliDiagnostic::error(format!("Failed to apply configuration: {}", e)))?;

        // Process files based on VCS configuration if applicable
        let file_paths = if self.changed || self.staged || self.since.is_some() {
            self.get_vcs_files(workspace)?
        } else if self.paths.is_empty() {
            vec![workspace.root().into()]
        } else {
            self.paths.clone()
        };

        for path in file_paths {
            let path = Path::new(&path);
            if path.is_file() {
                self.process_file(path, console, &formatter)?;
            } else if path.is_dir() {
                let glob_pattern = format!("{}/**/*.md", path.display());
                let glob = Glob::from_str(&glob_pattern)
                    .map_err(|e| CliDiagnostic::error(format!("Invalid glob pattern: {}", e)))?;

                for entry in walkdir::WalkDir::new(path) {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_file() && glob.is_match(path.to_str().unwrap_or_default()) {
                                self.process_file(path, console, &formatter)?;
                            }
                        }
                        Err(e) => {
                            console.log(&format!("Error processing entry: {}", e));
                        }
                    }
                }
            }
        }

        // Handle stdin if provided
        if let Some(_stdin_path) = &self.stdin_file_path {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .map_err(|e| CliDiagnostic::error(format!("Failed to read from stdin: {}", e)))?;

            let formatted = formatter
                .format_content(&buffer)
                .map_err(|e| CliDiagnostic::error(format!("Failed to format content: {}", e)))?;
            console.log(&formatted);
        }

        console.log("\nFormat operation completed successfully!");
        Ok(())
    }
}

impl FormatCommand {
    pub fn new(
        write: bool,
        fix: bool,
        paths: Vec<OsString>,
        stdin_file_path: Option<String>,
    ) -> Self {
        Self {
            write,
            fix,
            paths,
            stdin_file_path,
            show_help: false,
            formatter_config: PartialMarkdownFormatterConfiguration::default(),
            staged: false,
            changed: false,
            since: None,
        }
    }

    pub fn with_help() -> Self {
        Self {
            show_help: true,
            ..Default::default()
        }
    }

    fn get_vcs_files(&self, _workspace: &Workspace) -> Result<Vec<OsString>, CliDiagnostic> {
        // TODO: Implement VCS file detection using git2 or similar
        Ok(vec![])
    }

    fn process_file(
        &self,
        path: &Path,
        console: &impl Console,
        formatter: &MarkdownFormatter,
    ) -> Result<(), CliDiagnostic> {
        if !path.exists() {
            return Err(CliDiagnostic::error(format!("File not found: {:?}", path)));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| CliDiagnostic::error(format!("Failed to read file {:?}: {}", path, e)))?;

        let formatted = formatter.format_content(&content).map_err(|e| {
            CliDiagnostic::error(format!("Failed to format file {:?}: {}", path, e))
        })?;

        if self.write || self.fix {
            fs::write(path, formatted).map_err(|e| {
                CliDiagnostic::error(format!("Failed to write file {:?}: {}", path, e))
            })?;
            console.log(&format!("Formatted {:?}", path));
        } else {
            console.log(&formatted);
        }

        Ok(())
    }

    fn print_help(console: &impl Console) {
        console.log("Run the formatter on a set of files.");
        console.log("");
        console.log("Usage: ripari format [--write] [PATH]...");
        console.log("");
        console.log("Formatting options:");
        console.log("        --write               Write formatted files to disk");
        console
            .log("        --fix                 Alias of --write, writes formatted files to disk");
        console.log("        --stdin-file-path=PATH Use this option when you want to format code");
        console
            .log("                              piped from stdin, and print the output to stdout");
        console.log("");
        console.log("Available positional items:");
        console.log("    PATH                      Single file, single path or list of paths");
        console.log("");
        console.log("Available options:");
        console.log("    -h, --help               Prints help information");
    }
}
