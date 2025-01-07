pub mod format;
pub mod lint;

use crate::console::Console;
use crate::workspace::Workspace;
use crate::diagnostics::CliDiagnostic;

pub enum SimoraCommand {
    Format(format::FormatCommand),
    Lint(lint::LintCommand),
    ShowHelp,
}

impl SimoraCommand {
    pub fn print_usage(console: &impl Console) {
        console.log("Simora Prompt official CLI. Use it to check and format your code.");
        console.log("");
        console.log("Usage: simora-prompt COMMAND ...");
        console.log("");
        console.log("Available options:");
        console.log("    -h, --help     Prints help information");
        console.log("    -V, --version  Prints version information");
        console.log("");
        console.log("Available commands:");
        console.log("    format         Run the formatter on a set of files");
        console.log("                   Format your code according to the configured style");
        console.log("    lint           Run the linter on a set of files");
        console.log("                   Check your code for potential errors and style violations");
    }

    pub fn from_args() -> Result<Self, CliDiagnostic> {
        let args: Vec<String> = std::env::args().skip(1).collect();

        if args.is_empty() {
            return Ok(SimoraCommand::ShowHelp);
        }

        // Basic command parsing
        match args[0].as_str() {
            "format" => {
                if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
                    Ok(SimoraCommand::Format(format::FormatCommand::with_help()))
                } else {
                    Ok(SimoraCommand::Format(format::FormatCommand::default()))
                }
            },
            "lint" => Ok(SimoraCommand::Lint(lint::LintCommand::default())),
            "-h" | "--help" => Ok(SimoraCommand::ShowHelp),
            _ => Err(CliDiagnostic::new("Unknown command. Use --help for usage information.")),
        }
    }

    pub fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        match self {
            SimoraCommand::Format(cmd) => cmd.execute(console, workspace),
            SimoraCommand::Lint(cmd) => cmd.execute(console, workspace),
            SimoraCommand::ShowHelp => {
                Self::print_usage(console);
                Ok(())
            }
        }
    }
}
