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

    pub fn error<T: Into<String>>(message: T) -> Self {
        Self::new(message)
    }
}

impl fmt::Display for CliDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliDiagnostic {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let diagnostic = CliDiagnostic::new("test message");
        assert_eq!(diagnostic.message, "test message");
    }

    #[test]
    fn test_error_creation() {
        let error = CliDiagnostic::error("error message");
        assert_eq!(error.message, "error message");
    }

    #[test]
    fn test_display_implementation() {
        let diagnostic = CliDiagnostic::new("test message");
        assert_eq!(diagnostic.to_string(), "test message");
    }

    #[test]
    fn test_error_trait_implementation() {
        let error: Box<dyn std::error::Error> = Box::new(CliDiagnostic::error("test error"));
        assert_eq!(error.to_string(), "test error");
    }
}
