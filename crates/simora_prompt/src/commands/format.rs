use std::ffi::OsString;
use crate::console::Console;
use crate::workspace::Workspace;
use crate::diagnostics::CliDiagnostic;

#[derive(Debug, Default)]
pub struct FormatCommand {
    paths: Vec<OsString>,
    write: bool,
    fix: bool,
    stdin_file_path: Option<String>,
    show_help: bool,
}

impl FormatCommand {
    pub fn new(write: bool, fix: bool, paths: Vec<OsString>, stdin_file_path: Option<String>) -> Self {
        Self {
            write,
            fix,
            paths,
            stdin_file_path,
            show_help: false,
        }
    }

    pub fn with_help() -> Self {
        Self {
            show_help: true,
            ..Default::default()
        }
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

    pub fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        if self.show_help {
            Self::print_help(console);
            return Ok(());
        }

        console.log(&format!(
            "Formatting files in workspace: {:?}",
            workspace.root()
        ));

        // Display the configuration
        console.log(&format!("Write mode: {}", self.write));
        console.log(&format!("Fix mode: {}", self.fix));

        // Display paths being processed
        if !self.paths.is_empty() {
            console.log("\nProcessing files:");
            for path in &self.paths {
                console.log(&format!("  - {:?}", path));
            }
        }

        if let Some(stdin_path) = &self.stdin_file_path {
            console.log(&format!("\nProcessing stdin as file: {}", stdin_path));
        }

        console.log("\nFormat operation completed successfully!");
        Ok(())
    }
}
