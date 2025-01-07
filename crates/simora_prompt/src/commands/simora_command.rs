use crate::console::Console;
use crate::workspace::Workspace;
use crate::diagnostics::CliDiagnostic;
use super::{format, lint};

#[derive(Debug)]
pub enum SimoraCommand {
    Format(format::FormatCommand),
    Lint(lint::LintCommand),
}

impl SimoraCommand {
    pub fn from_args() -> Result<Self, String> {
        // Simulate argument parsing; default to FormatCommand for this example
        Ok(SimoraCommand::Format(format::FormatCommand::default()))
    }

    pub fn execute(&self, console: &mut impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        match self {
            SimoraCommand::Format(cmd) => cmd.execute(console, workspace),
            SimoraCommand::Lint(cmd) => cmd.execute(console, workspace),
        }
    }
}
