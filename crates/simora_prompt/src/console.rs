pub trait Console {
    fn log(&self, message: &str);
    fn error(&self, message: &str);
}

#[derive(Debug)]
pub struct EnvConsole {
    verbose: bool,
}

impl EnvConsole {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl Console for EnvConsole {
    fn log(&self, message: &str) {
        if self.verbose {
            println!("{}", message);
        }
    }

    fn error(&self, message: &str) {
        eprintln!("Error: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::sync::Mutex;
    use std::cell::RefCell;
    // use std::rc::Rc;

    #[derive(Default)]
    struct TestConsole {
        logs: RefCell<Vec<String>>,
        errors: RefCell<Vec<String>>,
    }

    impl Console for TestConsole {
        fn log(&self, message: &str) {
            self.logs.borrow_mut().push(message.to_string());
        }

        fn error(&self, message: &str) {
            self.errors.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_env_console_verbose() {
        let console = EnvConsole::new(true);
        assert!(console.verbose);
    }

    #[test]
    fn test_env_console_non_verbose() {
        let console = EnvConsole::new(false);
        assert!(!console.verbose);
    }

    #[test]
    fn test_console_logging() {
        let console = TestConsole::default();
        console.log("test message");
        assert_eq!(console.logs.borrow().len(), 1);
        assert_eq!(console.logs.borrow()[0], "test message");
    }

    #[test]
    fn test_console_error() {
        let console = TestConsole::default();
        console.error("error message");
        assert_eq!(console.errors.borrow().len(), 1);
        assert_eq!(console.errors.borrow()[0], "error message");
    }

    #[test]
    fn test_multiple_messages() {
        let console = TestConsole::default();
        console.log("message 1");
        console.log("message 2");
        console.error("error 1");
        console.error("error 2");

        assert_eq!(console.logs.borrow().len(), 2);
        assert_eq!(console.errors.borrow().len(), 2);
        assert_eq!(console.logs.borrow().join(", "), "message 1, message 2");
        assert_eq!(console.errors.borrow().join(", "), "error 1, error 2");
    }
}
