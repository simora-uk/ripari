use std::ffi::OsString;
use crate::console::Console;
use crate::workspace::Workspace;
use crate::diagnostics::CliDiagnostic;

#[derive(Debug, Default)]
pub struct LintCommand {
    paths: Vec<OsString>,
    write: bool,
    fix: bool,
    stdin_file_path: Option<String>,
}

impl LintCommand {
    pub fn new(write: bool, fix: bool, paths: Vec<OsString>, stdin_file_path: Option<String>) -> Self {
        Self { write, fix, paths, stdin_file_path }
    }

    pub fn execute(&self, console: &impl Console, workspace: &Workspace) -> Result<(), CliDiagnostic> {
        console.log(&format!(
            "Linting files in workspace: {:?}",
            workspace.root()
        ));
        Ok(())
    }
}
