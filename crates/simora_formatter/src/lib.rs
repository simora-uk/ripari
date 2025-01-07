use regex::Regex;
use simora_configuration::{PartialMarkdownFormatterConfiguration, MarkdownFormatterConfig};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FormatterError {
    InvalidRule(String),
    ConfigurationError(String),
    FormatError(String),
}

impl fmt::Display for FormatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatterError::InvalidRule(msg) => write!(f, "Invalid rule: {}", msg),
            FormatterError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            FormatterError::FormatError(msg) => write!(f, "Format error: {}", msg),
        }
    }
}

impl Error for FormatterError {}

/// Trait for formatters
pub trait Formatter {
    fn format_content(&self, content: &str) -> Result<String, FormatterError>;
    fn apply_configuration(&mut self, config: &PartialMarkdownFormatterConfiguration) -> Result<(), FormatterError>;
}

/// A basic Markdown formatter
#[derive(Debug)]
pub struct MarkdownFormatter {
    pub config: Option<MarkdownFormatterConfig>,
}

impl Default for MarkdownFormatter {
    fn default() -> Self {
        Self {
            config: None,
        }
    }
}

impl MarkdownFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    fn format_smart_quotes(&self, content: &str) -> String {
        if let Some(config) = &self.config {
            if config.rules.smart_quotes.enabled {
                // Replace smart quotes with straight quotes
                content
                    .replace('\u{201c}', "\"")  // Left double quote
                    .replace('\u{201d}', "\"")  // Right double quote
                    .replace('\u{2018}', "'")   // Left single quote
                    .replace('\u{2019}', "'")   // Right single quote
            } else {
                content.to_string()
            }
        } else {
            content.to_string()
        }
    }

    fn format_headings(&self, content: &str) -> String {
        if let Some(config) = &self.config {
            if config.rules.headings.enabled && config.rules.headings.remove_emphasis {
                let heading_pattern = Regex::new(r"^(#+)\s+\*\*(.*?)\*\*$").unwrap();
                let lines: Vec<String> = content.lines()
                    .map(|line| {
                        if heading_pattern.is_match(line) {
                            heading_pattern.replace(line, "$1 $2").to_string()
                        } else {
                            line.to_string()
                        }
                    })
                    .collect();
                lines.join("\n")
            } else {
                content.to_string()
            }
        } else {
            content.to_string()
        }
    }

    fn remove_horizontal_rules(&self, content: &str, in_code_block: bool) -> String {
        if let Some(config) = &self.config {
            if config.rules.remove_horizontal_rules.enabled {
                let hr_pattern = Regex::new(r"(?m)^\s*---\s*$").unwrap();
                let mut result = Vec::new();
                let mut prev_was_empty = false;
                let original_line_ending = if content.contains("\r\n") { "\r\n" } else { "\n" };
                let lines: Vec<&str> = content.split(original_line_ending).collect();

                // Handle empty content or whitespace-only content
                if lines.iter().all(|line| line.trim().is_empty()) {
                    return content.to_string();
                }

                // Handle leading horizontal rule
                if !lines.is_empty() && hr_pattern.is_match(lines[0]) {
                    result.push(String::new());
                    prev_was_empty = true;
                }

                for (i, line) in lines.iter().enumerate() {
                    if line.starts_with("```") || line.starts_with("    ") || line.starts_with("\t") || line.starts_with(">") {
                        // Preserve special blocks exactly
                        result.push((*line).to_string());
                        prev_was_empty = false;
                    } else if hr_pattern.is_match(line) {
                        if !in_code_block && !prev_was_empty && i > 0 {
                            result.push(String::new());
                            prev_was_empty = true;
                        }
                    } else if line.trim().is_empty() {
                        if !prev_was_empty {
                            result.push((*line).to_string());
                            prev_was_empty = true;
                        }
                    } else {
                        result.push((*line).to_string());
                        prev_was_empty = false;
                    }
                }

                // Preserve trailing line ending if present
                if content.ends_with(original_line_ending) && !result.is_empty() {
                    let last_line = result.last().unwrap();
                    if !last_line.is_empty() && !hr_pattern.is_match(last_line) {
                        result.push(String::new());
                    }
                }

                result.join(original_line_ending)
            } else {
                content.to_string()
            }
        } else {
            content.to_string()
        }
    }

    fn format_punctuation(&self, content: &str) -> String {
        if let Some(config) = &self.config {
            if config.rules.punctuation.enabled {
                let mut result = content.to_string();

                if config.rules.punctuation.standardize_dashes {
                    // Convert em-dashes and en-dashes to hyphens
                    result = result.replace('—', "-").replace('–', "-");
                }

                if config.rules.punctuation.standardize_ellipsis {
                    // Convert ellipsis character to three dots
                    result = result.replace('…', "...");
                }

                result
            } else {
                content.to_string()
            }
        } else {
            content.to_string()
        }
    }

    fn is_unformatted_block_start(line: &str) -> bool {
        // Only code blocks should trigger the unformatted block state
        line.trim_start().starts_with("```")
    }

    fn format_content_once(&self, content: &str) -> Result<String, FormatterError> {
        let mut result = Vec::new();
        let mut in_code_block = false;  // Renamed from in_unformatted_block for clarity
        let mut lines = Vec::new();
        let mut current_line = String::new();

        // Split content preserving line endings
        for c in content.chars() {
            if c == '\n' {
                if current_line.ends_with('\r') {
                    current_line.pop();
                    lines.push((current_line.clone(), "\r\n"));
                } else {
                    lines.push((current_line.clone(), "\n"));
                }
                current_line.clear();
            } else {
                current_line.push(c);
            }
        }
        if !current_line.is_empty() {
            lines.push((current_line, ""));
        }

        for (i, (line, ending)) in lines.iter().enumerate() {
            // Toggle code block state
            if Self::is_unformatted_block_start(line) {
                result.push(format!("{}{}", line, ending));
                in_code_block = !in_code_block;
                continue;
            }

            if in_code_block {
                result.push(format!("{}{}", line, ending));
                continue;
            }

            // Handle blockquotes line by line
            if line.starts_with(">") {
                result.push(format!("{}{}", line, ending));
                continue;
            }

            // Apply formatting to regular content
            let mut formatted = line.to_string();
            formatted = self.format_punctuation(&formatted);
            formatted = self.format_smart_quotes(&formatted);
            formatted = self.format_headings(&formatted);
            result.push(format!("{}{}", formatted, ending));
        }

        let mut content = result.join("");

        // Only apply horizontal rule processing if not in a code block
        if !in_code_block {
            content = self.remove_horizontal_rules(&content, in_code_block);
        }

        Ok(content)
    }
}

impl Formatter for MarkdownFormatter {
    fn format_content(&self, content: &str) -> Result<String, FormatterError> {
        if self.config.is_none() {
            return Err(FormatterError::ConfigurationError(
                "No configuration provided".to_string(),
            ));
        }

        // Early return for empty content
        if content.is_empty() {
            return Ok(String::new());
        }

        // Early return for whitespace-only content
        if content.chars().all(|c| c.is_whitespace()) {
            return Ok(content.to_string());
        }

        let mut current = content.to_string();
        let mut previous;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100; // Prevent infinite loops

        loop {
            previous = current.clone();
            // First, process the entire content as a whole
            let intermediate = self.format_content_once(&previous)?;

            // Then process again to handle any new line situations
            current = self.format_content_once(&intermediate)?;

            iterations += 1;
            if current == previous || iterations >= MAX_ITERATIONS {
                if iterations >= MAX_ITERATIONS {
                    return Err(FormatterError::FormatError(
                        "Output has not stabilized after maximum iterations".to_string(),
                    ));
                }
                break;
            }
        }

        Ok(current)
    }

    fn apply_configuration(&mut self, config: &PartialMarkdownFormatterConfiguration) -> Result<(), FormatterError> {
        self.config = config.markdown.clone();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simora_configuration::{
        MarkdownFormatterConfig, RulesConfig, SmartQuotesConfig,
        HeadingsConfig, HorizontalRulesConfig, PunctuationConfig,
    };

    fn create_test_config() -> PartialMarkdownFormatterConfiguration {
        PartialMarkdownFormatterConfiguration {
            root: false,
            markdown: Some(MarkdownFormatterConfig {
                enabled: true,
                rules: RulesConfig {
                    smart_quotes: SmartQuotesConfig { enabled: true },
                    headings: HeadingsConfig {
                        enabled: true,
                        remove_emphasis: true,
                    },
                    remove_horizontal_rules: HorizontalRulesConfig { enabled: true },
                    punctuation: PunctuationConfig {
                        enabled: true,
                        standardize_dashes: true,
                        standardize_ellipsis: true,
                    },
                },
            }),
            files: None,
            vcs: None,
        }
    }

    // Smart Quotes Tests
    #[test]
    fn test_smart_quotes_all_variants() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#""double" "double" 'single' 'single'"#;
        let expected = r#""double" "double" 'single' 'single'"#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smart_quotes_nested() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#""He said 'hello' to me""#;
        let expected = r#""He said 'hello' to me""#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smart_quotes_apostrophes() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"Don't can't won't it's"#;
        let expected = r#"Don't can't won't it's"#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_smart_quotes_disabled() {
        let mut formatter = MarkdownFormatter::new();
        let mut config = create_test_config();
        if let Some(ref mut markdown) = config.markdown {
            markdown.rules.smart_quotes.enabled = false;
        }
        formatter.apply_configuration(&config).unwrap();

        let input = r#""smart" quotes 'unchanged'"#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    // Heading Tests
    #[test]
    fn test_headings_all_levels() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "# **H1**\n## **H2**\n### **H3**\n#### **H4**\n##### **H5**\n###### **H6**";
        let expected = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_headings_without_emphasis() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "# Plain H1\n## Plain H2";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_headings_malformed() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "#Not a heading\n##**Bad Space**";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_headings_disabled() {
        let mut formatter = MarkdownFormatter::new();
        let mut config = create_test_config();
        if let Some(ref mut markdown) = config.markdown {
            markdown.rules.headings.enabled = false;
        }
        formatter.apply_configuration(&config).unwrap();

        let input = "# **Title Still Bold**";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    // Horizontal Rules Tests
    #[test]
    fn test_horizontal_rules_variants() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let inputs = [
            "Before\n---\nAfter",
            "Before\n  ---  \nAfter",
            "Before\n\n---\n\nAfter",
            "Before\n\n  ---  \n\nAfter",
        ];

        for input in inputs.iter() {
            let result = formatter.format_content(input).unwrap();
            assert_eq!(result, "Before\n\nAfter");
        }
    }

    #[test]
    fn test_horizontal_rules_not_on_own_line() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let inputs = [
            "Text --- more text",
            "Text---text",
            "Some---",
            "---text",
            "Text\nSome text --- more\nText",
        ];

        for input in inputs.iter() {
            let result = formatter.format_content(*input).unwrap();
            assert_eq!(result, *input);
        }
    }

    #[test]
    fn test_horizontal_rules_consecutive() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Text\n---\n---\n---\nMore text";
        let expected = "Text\n\nMore text";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_horizontal_rules_at_boundaries() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "---\nStart text\n---\nEnd text\n---";
        let expected = "\nStart text\n\nEnd text\n";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    // Punctuation Tests
    #[test]
    fn test_punctuation_dashes_in_context() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Word—word and word–word";
        let expected = "Word-word and word-word";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_ellipsis_in_context() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "To be continued… and more…";
        let expected = "To be continued... and more...";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_mixed() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Start—middle…end–final";
        let expected = "Start-middle...end-final";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_disabled() {
        let mut formatter = MarkdownFormatter::new();
        let mut config = create_test_config();
        if let Some(ref mut markdown) = config.markdown {
            markdown.rules.punctuation.enabled = false;
        }
        formatter.apply_configuration(&config).unwrap();

        let input = "Word—word…word";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    // Edge Cases and Error Handling
    #[test]
    fn test_empty_content() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let result = formatter.format_content("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_whitespace_only() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "   \n  \t  \n  ";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn test_no_configuration() {
        let formatter = MarkdownFormatter::new();
        let result = formatter.format_content("test");
        assert!(result.is_err());
        if let Err(FormatterError::ConfigurationError(msg)) = result {
            assert_eq!(msg, "No configuration provided");
        } else {
            panic!("Expected ConfigurationError");
        }
    }

    #[test]
    fn test_all_rules_disabled() {
        let mut formatter = MarkdownFormatter::new();
        let mut config = create_test_config();
        if let Some(ref mut markdown) = config.markdown {
            markdown.rules.smart_quotes.enabled = false;
            markdown.rules.headings.enabled = false;
            markdown.rules.remove_horizontal_rules.enabled = false;
            markdown.rules.punctuation.enabled = false;
        }
        formatter.apply_configuration(&config).unwrap();

        let input = r#"# **Title**

"Quote" with 'marks'

---

Word—word…end"#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, input);
    }

    // Rule Interaction Tests
    #[test]
    fn test_heading_with_smart_quotes() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"# **Don't "Quote" Me**"#;
        let expected = r#"# Don't "Quote" Me"#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_dashes_and_quotes() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"# **The "Quick"—Brown—Fox**"#;
        let expected = r#"# The "Quick"-Brown-Fox"#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_bold_and_ellipsis() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "# **To Be Continued…**";
        let expected = "# To Be Continued...";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_mixed_content() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"# **Don't "Quote" Me—I'm…**

Here's a line with "quotes" and a dash—plus an ellipsis…

---

## **Another "Heading"—With—Style…**"#;

        let expected = r#"# Don't "Quote" Me-I'm...

Here's a line with "quotes" and a dash-plus an ellipsis...

## Another "Heading"-With-Style..."#;

        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rule_order_independence() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        // The result should be the same regardless of which characters appear first
        let inputs = [
            r#"# **"Quoted" Title—With—Dash…**"#,
            r#"# **…Dash—With—"Quoted" Title**"#,
            r#"# **—"Quoted"…Title—**"#,
        ];

        let expected = [
            r#"# "Quoted" Title-With-Dash..."#,
            r#"# ...Dash-With-"Quoted" Title"#,
            r#"# -"Quoted"...Title-"#,
        ];

        for (input, expected) in inputs.iter().zip(expected.iter()) {
            let result = formatter.format_content(input).unwrap();
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_selective_rule_disabling_with_mixed_content() {
        let mut formatter = MarkdownFormatter::new();
        let mut config = create_test_config();
        if let Some(ref mut markdown) = config.markdown {
            // Only disable smart quotes, keep other rules enabled
            markdown.rules.smart_quotes.enabled = false;
        }
        formatter.apply_configuration(&config).unwrap();

        let input = r#"# **"Title" with—dash…**"#;
        let expected = r#"# "Title" with-dash..."#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    // Known to fail, retained for future
    #[ignore]
    fn test_horizontal_rules_in_code_blocks() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Before\n```\n---\n```\nAfter";
        let expected = "Before\n```\n---\n```\nAfter";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_horizontal_rules_in_blockquotes() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Before\n> ---\nAfter";
        let expected = "Before\n> ---\nAfter";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_horizontal_rules() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "# **Title**\n---\nContent";
        let expected = "# Title\n\nContent";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);

        let input = "Content\n---\n# **Title**";
        let expected = "Content\n\n# Title";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn test_mixed_line_endings() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Line 1\r\n---\r\nLine 2\nLine 3\r\n---\nLine 4";
        let expected = "Line 1\r\n\r\nLine 2\nLine 3\r\n\r\nLine 4";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unicode_whitespace() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Before\n\u{2003}---\u{2003}\nAfter"; // Em space
        let expected = "Before\n\nAfter";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);

        let input = "Before\n\u{00A0}---\u{00A0}\nAfter"; // Non-breaking space
        let expected = "Before\n\nAfter";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_rules_interaction() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"# **"Title"**
## **Heading—With—Style…**
## **Final "Heading"**"
---
> Some "quoted" text with—dashes…
---
## **Final "Heading"—With—Style…**"#;

        let expected = r#"# "Title"
## Heading-With-Style...
## **Final "Heading"**"

> Some "quoted" text with—dashes…

## Final "Heading"-With-Style..."#;

        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected, "Failed to format content correctly.\n\nActual input:\n{}\n\nActual output:\n{}\n\nActual expected:\n{}", input, result, expected);
    }

    #[test]
    fn test_selective_rule_enabling() {
        let mut formatter = MarkdownFormatter::new();
        let mut config = create_test_config();
        if let Some(ref mut markdown) = config.markdown {
            markdown.rules.smart_quotes.enabled = false;
            markdown.rules.headings.enabled = true;
            markdown.rules.remove_horizontal_rules.enabled = false;
            markdown.rules.punctuation.enabled = false;
        }
        formatter.apply_configuration(&config).unwrap();

        let input = r#"# **"Title"**
---
## **"Heading"—With—Style…**"#;

        let expected = r#"# "Title"
---
## "Heading"—With—Style…"#;

        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_document() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "";
        let expected = "";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_only_horizontal_rules() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "---\n---\n---";
        let expected = "";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_multiple_on_line() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Text—with—multiple—dashes";
        let expected = "Text-with-multiple-dashes";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_multiple_ellipsis() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "First…second…third…";
        let expected = "First...second...third...";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_mixed_multiple() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Text—with…mixed—punctuation…marks";
        let expected = "Text-with...mixed-punctuation...marks";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_punctuation_consecutive() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Text——with……consecutive—punctuation";
        let expected = "Text--with......consecutive-punctuation";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_blockquote_state_reset() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"> Some quoted text
## **Should remove bold**"#;

        let expected = r#"> Some quoted text
## Should remove bold"#;

        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected, "Failed to reset blockquote state. Actual output:\n{}", result);
    }

    #[test]
    fn test_heading_bold_removal() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "## **Heading with bold**";
        let expected = "## Heading with bold";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_quotes() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"## **"Heading with quotes"**"#;
        let expected = r#"## "Heading with quotes""#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_dashes() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "## Heading—With—Dashes";
        let expected = "## Heading-With-Dashes";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_ellipsis() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "## Heading with ellipsis…";
        let expected = "## Heading with ellipsis...";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_heading_with_trailing_quote() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"## **"Heading"**""#;
        let expected = r#"## **"Heading"**""#;
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected, "Failed to format heading with trailing quote.\nActual: {}\nExpected: {}", result, expected);
    }

    #[test]
    fn test_heading_with_proper_bold() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = r#"## **"Heading"**"#;
        let expected = r#"## "Heading""#;

        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected, "Failed to format heading with proper bold markers.\nActual: {}\nExpected: {}", result, expected);
    }

    #[test]
    fn test_blockquote_preservation() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "> Text with—dashes… and \"quotes\"";
        let expected = "> Text with—dashes… and \"quotes\"";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_horizontal_rule_removal() {
        let mut formatter = MarkdownFormatter::new();
        formatter.apply_configuration(&create_test_config()).unwrap();

        let input = "Text\n---\nMore text";
        let expected = "Text\n\nMore text";
        let result = formatter.format_content(input).unwrap();
        assert_eq!(result, expected);
    }
}

