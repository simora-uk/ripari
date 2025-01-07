use std::ffi::OsString;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::cell::RefCell;

use crate::console::Console;
use crate::diagnostics::CliDiagnostic;
use crate::workspace::Workspace;
use crate::globby::GlobMatcher;

/// Represents a single formatting rule with its configuration
#[derive(Debug, Clone)]
pub struct FormatRule {
    id: String,
    pattern: Regex,
    replacement: String,
    is_safe: bool,
    description: String,
}

/// Configuration for the formatter
#[derive(Debug, Clone, Default)]
pub struct FormatterConfiguration {
    pub indent_style: String,
    pub indent_width: usize,
    pub line_width: usize,
    pub use_tabs: bool,
    pub enabled_rules: Vec<String>,
}

/// VCS-specific configuration
#[derive(Debug, Clone, Default)]
pub struct VcsConfiguration {
    pub ignore_untracked: bool,
    pub ignore_ignored: bool,
    pub only_staged: bool,
}

#[derive(Debug, Default)]
pub struct FormatCommand {
    paths: Vec<OsString>,
    write: bool,
    fix: bool,
    stdin_file_path: Option<String>,
    show_help: bool,
    formatter_config: FormatterConfiguration,
    vcs_config: VcsConfiguration,
    staged: bool,
    changed: bool,
    since: Option<String>,
}

/// Trait for commands that can load editor configuration
pub trait LoadEditorConfig {
    fn load_editor_config(&self) -> Result<FormatterConfiguration, CliDiagnostic>;
}

/// Trait for executable commands
pub trait CommandRunner {
    fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic>;
}

impl LoadEditorConfig for FormatCommand {
    fn load_editor_config(&self) -> Result<FormatterConfiguration, CliDiagnostic> {
        // TODO: Implement .editorconfig loading
        Ok(FormatterConfiguration::default())
    }
}

impl CommandRunner for FormatCommand {
    fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        if self.show_help {
            Self::print_help(console);
            return Ok(());
        }

        // Load configuration
        let _config = self.load_editor_config()?;

        console.log(&format!(
            "Formatting files in workspace: {:?}",
            workspace.root()
        ));

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
                self.process_file(path, console)?;
            } else if path.is_dir() {
                let glob_pattern = format!("{}/**/*.md", path.display());
                let matcher = GlobMatcher::new(&glob_pattern)
                    .map_err(|e| CliDiagnostic::error(format!("Invalid glob pattern: {}", e)))?;

                for entry in matcher.walk(workspace.root()) {
                    match entry {
                        Ok(entry) if entry.path().is_file() => {
                            self.process_file(entry.path(), console)?;
                        }
                        Err(e) => {
                            console.log(&format!("Error processing entry: {}", e));
                        }
                        _ => {}
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

            let formatted = self.format_content(&buffer);
            console.log(&formatted);
        }

        console.log("\nFormat operation completed successfully!");
        Ok(())
    }
}

impl FormatCommand {
    pub fn new(write: bool, fix: bool, paths: Vec<OsString>, stdin_file_path: Option<String>) -> Self {
        Self {
            write,
            fix,
            paths,
            stdin_file_path,
            show_help: false,
            formatter_config: FormatterConfiguration {
                indent_style: String::from("space"),
                indent_width: 2,
                line_width: 80,
                use_tabs: false,
                enabled_rules: vec![],
            },
            vcs_config: VcsConfiguration::default(),
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

    fn get_format_rules() -> Vec<FormatRule> {
        vec![
            FormatRule {
                id: String::from("smart-quotes"),
                pattern: Regex::new(r#"["""]|''"#).unwrap(),
                replacement: String::from("\""),
                is_safe: true,
                description: String::from("Replaces smart quotes with regular quotes"),
            },
            FormatRule {
                id: String::from("clean-headings"),
                pattern: Regex::new(r"^(#+)\s*[*_](.*?)[*_]\s*$").unwrap(),
                replacement: String::from("$1 $2"),
                is_safe: true,
                description: String::from("Cleans headings by removing asterisks and underscores"),
            },
            FormatRule {
                id: String::from("standardize-dashes"),
                pattern: Regex::new(r"[–—]").unwrap(),
                replacement: String::from("-"),
                is_safe: true,
                description: String::from("Replaces dashes with regular hyphens"),
            },
        ]
    }

    fn format_content(&self, content: &str) -> String {
        let rules = Self::get_format_rules();

        // Process each line separately for heading cleanup
        let formatted = content
            .lines()
            .map(|line| {
                let mut line_content = line.to_string();
                for rule in &rules {
                    if rule.is_safe {
                        line_content = rule.pattern
                            .replace_all(&line_content, rule.replacement.as_str())
                            .to_string();
                    }
                }
                line_content
            })
            .collect::<Vec<String>>()
            .join("\n");

        // Handle indentation
        let indent = if self.formatter_config.use_tabs {
            "\t".to_string()
        } else {
            " ".repeat(self.formatter_config.indent_width)
        };

        // Apply indentation and line width formatting
        formatted
            .lines()
            .map(|line| {
                if line.trim().starts_with("- ") || line.trim().starts_with("* ") {
                    format!("{}{}", indent, line.trim())
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn process_file(&self, path: &Path, console: &impl Console) -> Result<(), CliDiagnostic> {
        if !path.exists() {
            return Err(CliDiagnostic::error(format!("File not found: {:?}", path)));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| CliDiagnostic::error(format!("Failed to read file {:?}: {}", path, e)))?;

        let formatted = self.format_content(&content);

        if self.write || self.fix {
            fs::write(path, formatted)
                .map_err(|e| CliDiagnostic::error(format!("Failed to write file {:?}: {}", path, e)))?;
            console.log(&format!("Formatted {:?}", path));
        } else {
            console.log(&formatted);
        }

        Ok(())
    }

    fn print_help(console: &impl Console) {
        console.log("Run the formatter on a set of files.");
        console.log("");
        console.log("Usage: simora-prompt format [--write] [PATH]...");
        console.log("");
        console.log("Formatting options:");
        console.log("        --write               Write formatted files to disk");
        console.log("        --fix                 Alias of --write, writes formatted files to disk");
        console.log("        --stdin-file-path=PATH Use this option when you want to format code");
        console.log("                              piped from stdin, and print the output to stdout");
        console.log("");
        console.log("Generic options:");
        console.log("        --indent-style=<tab|space>  The indent style");
        console.log("        --indent-width=NUMBER  The size of the indentation, 2 by default");
        console.log("        --line-width=NUMBER    What's the max width of a line. Defaults to 80");
        console.log("");
        console.log("Available positional items:");
        console.log("    PATH                      Single file, single path or list of paths");
        console.log("");
        console.log("Available options:");
        console.log("    -h, --help               Prints help information");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use tempfile::TempDir;

    /// A mock console implementation for testing
    #[derive(Default)]
    struct MockConsole {
        messages: RefCell<Vec<String>>,
    }

    impl Console for MockConsole {
        fn log(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }

        fn error(&self, message: &str) {
            self.messages.borrow_mut().push(format!("ERROR: {}", message));
        }
    }

    #[test]
    fn test_format_rules() {
        let cmd = FormatCommand::default();

        // Test smart quotes
        let input = r#"Here's some "smart" quotes and ''double'' quotes"#;
        let expected = r#"Here's some "smart" quotes and "double" quotes"#;
        assert_eq!(cmd.format_content(input), expected);

        // Test heading cleanup
        let input = "# *Important Heading*\n## _Another Heading_";
        let expected = "# Important Heading\n## Another Heading";
        assert_eq!(cmd.format_content(input), expected);

        // Test dash standardization
        let input = "Here's a dash — and another –";
        let expected = "Here's a dash - and another -";
        assert_eq!(cmd.format_content(input), expected);
    }

    #[test]
    fn test_indentation() {
        let mut cmd = FormatCommand::default();
        cmd.formatter_config.indent_width = 4;

        let input = "- First item\n* Second item";
        let expected = "    - First item\n    * Second item";
        assert_eq!(cmd.format_content(input), expected);

        // Test with tabs
        cmd.formatter_config.use_tabs = true;
        let expected = "\t- First item\n\t* Second item";
        assert_eq!(cmd.format_content(input), expected);
    }

    #[test]
    fn test_file_processing() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let _workspace = Workspace::new();
        let console = MockConsole::default();

        // Create a test file
        let test_file = temp_dir.path().join("test.md");
        fs::write(&test_file, "# *Test Heading*\n- Item 1\n- Item 2")?;

        let cmd = FormatCommand::new(
            true,
            false,
            vec![test_file.clone().into_os_string()],
            None,
        );

        // Process the file
        cmd.process_file(&test_file, &console)?;

        // Read the processed file
        let content = fs::read_to_string(&test_file)?;
        let expected = "# Test Heading\n  - Item 1\n  - Item 2";
        assert_eq!(content, expected);

        Ok(())
    }

    #[test]
    fn test_vcs_configuration() {
        let mut cmd = FormatCommand::default();
        cmd.vcs_config.only_staged = true;
        cmd.staged = true;

        // Test that VCS configuration is properly set
        assert!(cmd.vcs_config.only_staged);
        assert!(cmd.staged);
    }

    #[test]
    fn test_formatter_configuration() {
        let cmd = FormatCommand::new(false, false, vec![], None);

        // Test default configuration values
        assert_eq!(cmd.formatter_config.indent_style, "space");
        assert_eq!(cmd.formatter_config.indent_width, 2);
        assert_eq!(cmd.formatter_config.line_width, 80);
        assert!(!cmd.formatter_config.use_tabs);
        assert!(cmd.formatter_config.enabled_rules.is_empty());
    }

    #[test]
    fn test_help_output() {
        let console = MockConsole::default();
        let cmd = FormatCommand {
            show_help: true,
            ..Default::default()
        };

        cmd.execute(&console, &Workspace::new()).unwrap();

        // Verify help message contains expected content
        assert!(console.messages.borrow().iter().any(|msg| msg.contains("Usage: simora-prompt format")));
    }
}
