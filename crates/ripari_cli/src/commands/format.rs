use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::console::Console;
use crate::diagnostics::CliDiagnostic;
use crate::workspace::Workspace;
use simora_configuration:: PartialMarkdownFormatterConfiguration;
use simora_formatter::{Formatter, MarkdownFormatter};
use simora_glob::Glob;

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
    pub staged: bool,
    pub changed: bool,
    pub since: Option<String>,
    pub verbose: bool,
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

        // Set verbose mode
        simora_formatter::set_verbose(self.verbose);

        console.log(&format!(
            "Formatting files in workspace: {:?}",
            workspace.root()
        ));

        // Create formatter with global config
        let mut formatter = MarkdownFormatter::new();

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
        verbose: bool,
    ) -> Self {
        Self {
            write,
            fix,
            paths,
            stdin_file_path,
            show_help: false,
            staged: false,
            changed: false,
            since: None,
            verbose,
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

        console.log(&format!("Processing file: {:?}", path));
        console.log(&format!("Original content length: {}", content.len()));

        let formatted = formatter.format_content(&content).map_err(|e| {
            CliDiagnostic::error(format!("Failed to format file {:?}: {}", path, e))
        })?;

        console.log(&format!("Formatted content length: {}", formatted.len()));

        if content == formatted {
            console.log("Content is identical after formatting");
        } else {
            console.log("Content has changed after formatting");
            // Print first difference
            for (i, (original, formatted)) in content.chars().zip(formatted.chars()).enumerate() {
                if original != formatted {
                    console.log(&format!(
                        "First difference at position {}: original '{}' vs formatted '{}'",
                        i, original, formatted
                    ));
                    break;
                }
            }
        }

        if self.write || self.fix {
            if content != formatted {
                match fs::write(path, &formatted) {
                    Ok(_) => {
                        console.log(&format!("Formatted {:?}", path));
                    }
                    Err(e) => {
                        return Err(CliDiagnostic::error(format!(
                            "Failed to write file {:?}: {}. Please check file permissions.",
                            path, e
                        )));
                    }
                }
            } else {
                console.log(&format!("No changes needed for {:?}", path));
            }
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
        console.log("        --verbose             Show detailed debug information");
        console.log("");
        console.log("Available positional items:");
        console.log("    PATH                      Single file, single path or list of paths");
        console.log("");
        console.log("Available options:");
        console.log("    -h, --help               Prints help information");
    }
}
