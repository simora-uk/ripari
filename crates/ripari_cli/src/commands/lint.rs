use crate::console::Console;
use crate::diagnostics::CliDiagnostic;
use crate::workspace::Workspace;

#[derive(Debug, Default)]
pub struct LintCommand {
    // paths: Vec<OsString>,
    // write: bool,
    // fix: bool,
    // stdin_file_path: Option<String>,
}

impl LintCommand {
    pub fn new(

    ) -> Self {
        Self {
            // write,
            // fix,
            // paths,
            // stdin_file_path,
        }
    }

    pub fn execute(
        &self,
        console: &impl Console,
        workspace: &Workspace,
    ) -> Result<(), CliDiagnostic> {
        console.log(&format!(
            "Linting files in workspace: {:?}",
            workspace.root()
        ));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockConsole {
        logs: RefCell<Vec<String>>,
    }

    impl Console for MockConsole {
        fn log(&self, message: &str) {
            self.logs.borrow_mut().push(message.to_string());
        }

        fn error(&self, message: &str) {
            self.logs.borrow_mut().push(format!("ERROR: {}", message));
        }
    }

    impl MockConsole {
        fn new() -> Self {
            Self {
                logs: RefCell::new(Vec::new()),
            }
        }

        fn get_logs(&self) -> Vec<String> {
            self.logs.borrow().clone()
        }
    }


    #[test]
    fn test_lint_command_execution() {
        let console = MockConsole::new();
        let workspace = Workspace::new();
        let cmd = LintCommand::default();

        let result = cmd.execute(&console, &workspace);
        assert!(result.is_ok());

        let logs = console.get_logs();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("Linting files in workspace:"));
    }

}
