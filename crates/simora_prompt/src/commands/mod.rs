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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::ffi::OsString;

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
    fn test_command_from_args() {
        let cmd = SimoraCommand::from_args();
        assert!(cmd.is_ok());
        match cmd.unwrap() {
            SimoraCommand::Format(_) => (),
            _ => panic!("Expected Format command"),
        }
    }

    #[test]
    fn test_format_command_execution() {
        let console = MockConsole::new();
        let workspace = Workspace::new();
        let cmd = SimoraCommand::Format(format::FormatCommand::default());

        let result = cmd.execute(&console, &workspace);
        assert!(result.is_ok());

        let logs = console.get_logs();
        assert!(!logs.is_empty());
        assert!(logs[0].contains("Formatting files in workspace:"));
    }

    #[test]
    fn test_lint_command_execution() {
        let console = MockConsole::new();
        let workspace = Workspace::new();
        let cmd = SimoraCommand::Lint(lint::LintCommand::default());

        let result = cmd.execute(&console, &workspace);
        assert!(result.is_ok());

        let logs = console.get_logs();
        assert!(!logs.is_empty());
        assert!(logs[0].contains("Linting files in workspace:"));
    }

    #[test]
    fn test_command_debug_format() {
        let cmd = SimoraCommand::Format(format::FormatCommand::default());
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Format"));
    }
}
