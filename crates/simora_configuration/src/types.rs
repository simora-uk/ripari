use serde::{Deserialize, Serialize};
use super::rules::RulesConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarkdownFormatterConfig {
    pub enabled: bool,
    pub rules: RulesConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialFilesConfiguration {
    pub ignore: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialVcsConfiguration {
    pub enabled: Option<bool>,
    pub client_kind: Option<String>,
    pub use_ignore_file: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialMarkdownFormatterConfiguration {
    pub markdown: Option<MarkdownFormatterConfig>,
    pub files: Option<PartialFilesConfiguration>,
    pub vcs: Option<PartialVcsConfiguration>,
    #[serde(default)]
    pub root: bool,
}
