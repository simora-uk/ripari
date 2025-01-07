use std::fmt;

#[derive(Debug)]
pub struct CliDiagnostic {
    message: String,
}

impl CliDiagnostic {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CliDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliDiagnostic {}
