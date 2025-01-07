use regex::Regex;

/// Represents a single formatting rule with its configuration
#[derive(Debug, Clone)]
pub struct FormatRule {
    pub id: String,
    pub pattern: Regex,
    pub replacement: String,
    pub is_safe: bool,
    pub description: String,
}

/// Trait for formatters
pub trait Formatter {
    fn format_content(&self, content: &str) -> String;
}

/// A basic Markdown formatter
#[derive(Debug, Default)]
pub struct MarkdownFormatter {
    pub rules: Vec<FormatRule>,
}

impl Formatter for MarkdownFormatter {
    fn format_content(&self, content: &str) -> String {
        let mut formatted = content.to_string();
        for rule in &self.rules {
            if rule.is_safe {
                formatted = rule.pattern.replace_all(&formatted, &rule.replacement).to_string();
            }
        }
        formatted
    }
}

impl MarkdownFormatter {
    pub fn new(rules: Vec<FormatRule>) -> Self {
        Self { rules }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_format_content_safe_rule() {
        let rules = vec![
            FormatRule {
                id: "test_rule".to_string(),
                pattern: Regex::new(r"test").unwrap(),
                replacement: "replaced".to_string(),
                is_safe: true,
                description: "Test rule".to_string(),
            }
        ];
        let formatter = MarkdownFormatter::new(rules);
        let content = "This is a test string.";
        let formatted_content = formatter.format_content(content);
        assert_eq!(formatted_content, "This is a replaced string.");
    }

    #[test]
    fn test_format_content_unsafe_rule() {
        let rules = vec![
            FormatRule {
                id: "test_rule".to_string(),
                pattern: Regex::new(r"test").unwrap(),
                replacement: "replaced".to_string(),
                is_safe: false,
                description: "Test rule".to_string(),
            }
        ];
        let formatter = MarkdownFormatter::new(rules);
        let content = "This is a test string.";
        let formatted_content = formatter.format_content(content);
        assert_eq!(formatted_content, "This is a test string.");
    }

    #[test]
    fn test_format_content_multiple_rules() {
         let rules = vec![
            FormatRule {
                id: "test_rule_1".to_string(),
                pattern: Regex::new(r"test").unwrap(),
                replacement: "replaced".to_string(),
                is_safe: true,
                description: "Test rule 1".to_string(),
            },
            FormatRule {
                id: "test_rule_2".to_string(),
                pattern: Regex::new(r"string").unwrap(),
                replacement: "text".to_string(),
                is_safe: true,
                description: "Test rule 2".to_string(),
            }
        ];
        let formatter = MarkdownFormatter::new(rules);
        let content = "This is a test string.";
        let formatted_content = formatter.format_content(content);
        assert_eq!(formatted_content, "This is a replaced text.");
    }

    #[test]
    fn test_format_content_no_rules() {
        let rules = vec![];
        let formatter = MarkdownFormatter::new(rules);
        let content = "This is a test string.";
        let formatted_content = formatter.format_content(content);
        assert_eq!(formatted_content, "This is a test string.");
    }
}
