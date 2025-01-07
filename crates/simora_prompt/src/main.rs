mod console;
mod workspace;
mod commands;
mod diagnostics;

use std::process::ExitCode;
use console::{Console, EnvConsole};
use workspace::Workspace;
use commands::SimoraCommand;
use diagnostics::CliDiagnostic;

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
