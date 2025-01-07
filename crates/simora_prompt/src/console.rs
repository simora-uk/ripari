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

