use crate::console::Console;
use crate::diagnostics::CliDiagnostic;
use crate::workspace::Workspace;
use crate::commands::format::CommandRunner;

pub mod format;
pub mod lint;

#[derive(Debug)]
pub enum SimoraCommand {
    Format(format::FormatCommand),
    Lint(lint::LintCommand),
}

impl SimoraCommand {
    pub fn from_args() -> Result<Self, CliDiagnostic> {
        // TODO: Implement proper argument parsing
        Ok(SimoraCommand::Format(format::FormatCommand::with_help()))
    }

    pub fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        match self {
            SimoraCommand::Format(cmd) => cmd.execute(console, workspace),
            SimoraCommand::Lint(cmd) => cmd.execute(console, workspace),
        }
    }
}
