use serde::{Deserialize, Serialize};

/// Trait for merging configurations
pub trait Merge {
    fn merge_with(&mut self, other: Self);
}

/// Smart quotes rule configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SmartQuotesConfig {
    pub enabled: bool,
}

/// Headings rule configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeadingsConfig {
    pub enabled: bool,
    pub remove_emphasis: bool,
}

/// Horizontal rules configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HorizontalRulesConfig {
    pub enabled: bool,
}

/// Punctuation rule configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PunctuationConfig {
    pub enabled: bool,
    pub standardize_dashes: bool,
    pub standardize_ellipsis: bool,
}

/// Rules configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RulesConfig {
    pub smart_quotes: SmartQuotesConfig,
    pub headings: HeadingsConfig,
    pub remove_horizontal_rules: HorizontalRulesConfig,
    pub punctuation: PunctuationConfig,
}

/// Markdown formatter configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarkdownFormatterConfig {
    pub enabled: bool,
    pub rules: RulesConfig,
}

/// Main configuration structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialMarkdownFormatterConfiguration {
    pub markdown: Option<MarkdownFormatterConfig>,
    pub files: Option<PartialFilesConfiguration>,
    pub vcs: Option<PartialVcsConfiguration>,
    #[serde(default)]
    pub root: bool,
}

/// Files configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialFilesConfiguration {
    pub ignore: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}

/// VCS configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialVcsConfiguration {
    pub enabled: Option<bool>,
    pub client_kind: Option<String>,
    pub use_ignore_file: Option<bool>,
}

impl Merge for PartialMarkdownFormatterConfiguration {
    fn merge_with(&mut self, other: Self) {
        if let Some(markdown) = other.markdown {
            self.markdown = Some(match self.markdown.take() {
                Some(mut current) => {
                    current.enabled |= markdown.enabled;

                    if markdown.rules.smart_quotes.enabled {
                        current.rules.smart_quotes = markdown.rules.smart_quotes;
                    }
                    if markdown.rules.headings.enabled {
                        current.rules.headings = markdown.rules.headings;
                    }
                    if markdown.rules.remove_horizontal_rules.enabled {
                        current.rules.remove_horizontal_rules =
                            markdown.rules.remove_horizontal_rules;
                    }
                    if markdown.rules.punctuation.enabled {
                        current.rules.punctuation = markdown.rules.punctuation;
                    }
                    current
                }
                None => markdown,
            });
        }
        if let Some(files) = other.files {
            self.files = Some(match self.files.take() {
                Some(mut current) => {
                    if let Some(ignore) = files.ignore {
                        current.ignore = Some(ignore);
                    }
                    if let Some(include) = files.include {
                        current.include = Some(include);
                    }
                    current
                }
                None => files,
            });
        }
        if let Some(vcs) = other.vcs {
            self.vcs = Some(match self.vcs.take() {
                Some(mut current) => {
                    if let Some(enabled) = vcs.enabled {
                        current.enabled = Some(enabled);
                    }
                    if let Some(client_kind) = vcs.client_kind {
                        current.client_kind = Some(client_kind);
                    }
                    if let Some(use_ignore_file) = vcs.use_ignore_file {
                        current.use_ignore_file = Some(use_ignore_file);
                    }
                    current
                }
                None => vcs,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_configuration() {
        // Default configuration should have sensible defaults for formatting
        let config = PartialMarkdownFormatterConfiguration::default();

        // By default, no configuration is provided
        assert!(config.markdown.is_none());
        assert!(config.files.is_none());
        assert!(config.vcs.is_none());
        assert!(!config.root);
    }

    #[test]
    fn test_partial_configuration_override() {
        // Create a partial configuration that only specifies smart quotes
        let partial_config = PartialMarkdownFormatterConfiguration {
            markdown: Some(MarkdownFormatterConfig {
                enabled: true,
                rules: RulesConfig {
                    smart_quotes: SmartQuotesConfig { enabled: true },
                    ..Default::default() // All other rules should remain default (false)
                },
            }),
            ..Default::default()
        };

        // Verify only smart quotes are enabled
        let rules = &partial_config.markdown.unwrap().rules;
        assert!(rules.smart_quotes.enabled);
        assert!(!rules.headings.enabled);
        assert!(!rules.headings.remove_emphasis);
        assert!(!rules.remove_horizontal_rules.enabled);
        assert!(!rules.punctuation.enabled);
        assert!(!rules.punctuation.standardize_dashes);
        assert!(!rules.punctuation.standardize_ellipsis);
    }

    #[test]
    fn test_merge_preserves_defaults() {
        // Base config with all defaults
        let mut base = PartialMarkdownFormatterConfiguration::default();

        // Override with partial config that only sets smart quotes
        let override_config = PartialMarkdownFormatterConfiguration {
            markdown: Some(MarkdownFormatterConfig {
                enabled: true,
                rules: RulesConfig {
                    smart_quotes: SmartQuotesConfig { enabled: true },
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        base.merge_with(override_config);

        // Check that only smart quotes were enabled, everything else preserved defaults
        let markdown = base.markdown.unwrap();
        assert!(markdown.enabled);
        assert!(markdown.rules.smart_quotes.enabled);
        assert!(!markdown.rules.headings.enabled);
        assert!(!markdown.rules.headings.remove_emphasis);
        assert!(!markdown.rules.remove_horizontal_rules.enabled);
        assert!(!markdown.rules.punctuation.enabled);
    }

    #[test]
    fn test_merge_multiple_configs() {
        // Root config with base settings
        let mut base = PartialMarkdownFormatterConfiguration {
            markdown: Some(MarkdownFormatterConfig {
                enabled: true,
                rules: RulesConfig {
                    smart_quotes: SmartQuotesConfig { enabled: true },
                    ..Default::default()
                },
            }),
            root: true,
            ..Default::default()
        };

        // Project config overriding some settings
        let project_config = PartialMarkdownFormatterConfiguration {
            markdown: Some(MarkdownFormatterConfig {
                enabled: true,
                rules: RulesConfig {
                    headings: HeadingsConfig {
                        enabled: true,
                        remove_emphasis: true,
                    },
                    ..Default::default()
                },
            }),
            ..Default::default()
        };

        base.merge_with(project_config);

        let markdown = base.markdown.unwrap();
        assert!(markdown.enabled);
        // Smart quotes from root config preserved
        assert!(markdown.rules.smart_quotes.enabled);
        // Headings from project config applied
        assert!(markdown.rules.headings.enabled);
        assert!(markdown.rules.headings.remove_emphasis);
        // Other rules still default
        assert!(!markdown.rules.remove_horizontal_rules.enabled);
        assert!(!markdown.rules.punctuation.enabled);
    }

    #[test]
    fn test_files_configuration_merge() {
        let mut base = PartialMarkdownFormatterConfiguration {
            files: Some(PartialFilesConfiguration {
                ignore: Some(vec!["**/node_modules/**".to_string()]),
                include: None,
            }),
            ..Default::default()
        };

        let override_config = PartialMarkdownFormatterConfiguration {
            files: Some(PartialFilesConfiguration {
                ignore: None,
                include: Some(vec!["**/*.md".to_string()]),
            }),
            ..Default::default()
        };

        base.merge_with(override_config);

        let files = base.files.unwrap();
        // Original ignore patterns preserved
        assert_eq!(files.ignore.unwrap(), vec!["**/node_modules/**"]);
        // New include patterns added
        assert_eq!(files.include.unwrap(), vec!["**/*.md"]);
    }

    #[test]
    fn test_vcs_configuration_merge() {
        let mut base = PartialMarkdownFormatterConfiguration {
            vcs: Some(PartialVcsConfiguration {
                enabled: Some(true),
                client_kind: Some("git".to_string()),
                use_ignore_file: None,
            }),
            ..Default::default()
        };

        let override_config = PartialMarkdownFormatterConfiguration {
            vcs: Some(PartialVcsConfiguration {
                enabled: None,
                client_kind: None,
                use_ignore_file: Some(true),
            }),
            ..Default::default()
        };

        base.merge_with(override_config);

        let vcs = base.vcs.unwrap();
        // Original settings preserved
        assert_eq!(vcs.enabled, Some(true));
        assert_eq!(vcs.client_kind, Some("git".to_string()));
        // New setting added
        assert_eq!(vcs.use_ignore_file, Some(true));
    }

    #[test]
    fn test_configuration_serialization() {
        let config = PartialMarkdownFormatterConfiguration {
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
            files: Some(PartialFilesConfiguration {
                ignore: Some(vec!["**/node_modules/**".to_string()]),
                include: Some(vec!["**/*.md".to_string()]),
            }),
            vcs: Some(PartialVcsConfiguration {
                enabled: Some(true),
                client_kind: Some("git".to_string()),
                use_ignore_file: Some(true),
            }),
            root: false,
        };

        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: PartialMarkdownFormatterConfiguration =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            deserialized.markdown.as_ref().unwrap().enabled,
            config.markdown.as_ref().unwrap().enabled
        );
    }
}
