mod commands;
mod console;
mod diagnostics;
mod workspace;

use commands::SimoraCommand;
use console::{Console, EnvConsole};
use diagnostics::CliDiagnostic;
use std::process::ExitCode;
use workspace::Workspace;

fn main() -> ExitCode {
    let mut console = EnvConsole::new(true);
    let workspace = Workspace::default();

    match run_workspace(&mut console, &workspace) {
        Ok(_) => ExitCode::SUCCESS,
        Err(diagnostic) => {
            console.error(&diagnostic.to_string());
            ExitCode::FAILURE
        }
    }
}

fn run_workspace(console: &mut impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
    let command = SimoraCommand::from_args()?;

    command.execute(console, workspace)?;
    Ok(())
}
