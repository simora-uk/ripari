use crate::commands::format::CommandRunner;
use crate::console::Console;
use crate::diagnostics::CliDiagnostic;
use crate::workspace::Workspace;

pub mod format;
pub mod lint;

#[derive(Debug)]
pub enum SimoraCommand {
    Format(format::FormatCommand),
}

impl SimoraCommand {
    pub fn from_args() -> Result<Self, CliDiagnostic> {
        use std::env;
        use std::ffi::OsString;

        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Ok(SimoraCommand::Format(format::FormatCommand::with_help()));
        }

        match args[1].as_str() {
            "format" => {
                let mut write = false;
                let mut fix = false;
                let mut verbose = false;
                let mut paths = Vec::new();
                let mut stdin_file_path = None;

                let mut i = 2;
                while i < args.len() {
                    match args[i].as_str() {
                        "--write" => {
                            write = true;
                        }
                        "--fix" => {
                            fix = true;
                        }
                        "--verbose" => {
                            verbose = true;
                        }
                        "--stdin-file-path" => {
                            if i + 1 < args.len() {
                                stdin_file_path = Some(args[i + 1].clone());
                                i += 1;
                            }
                        }
                        "--help" | "-h" => {
                            return Ok(SimoraCommand::Format(format::FormatCommand::with_help()));
                        }
                        arg if arg.starts_with("--") => {
                            println!("Debug: skipping unknown flag: {}", arg);
                        }
                        _ => {
                            paths.push(OsString::from(&args[i]));
                        }
                    }
                    i += 1;
                }

                Ok(SimoraCommand::Format(format::FormatCommand::new(
                    write,
                    fix,
                    paths,
                    stdin_file_path,
                    verbose,
                )))
            }
            _ => Ok(SimoraCommand::Format(format::FormatCommand::with_help())),
        }
    }

    pub fn execute(
        &self,
        console: &impl Console,
        workspace: &Workspace,
    ) -> Result<(), CliDiagnostic> {
        match self {
            SimoraCommand::Format(cmd) => cmd.execute(console, workspace),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    // use std::ffi::OsString;

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
    fn test_command_debug_format() {
        let cmd = SimoraCommand::Format(format::FormatCommand::default());
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Format"));
    }
}
