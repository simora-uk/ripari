use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SmartQuotesConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeadingsConfig {
    pub enabled: bool,
    pub remove_emphasis: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HorizontalRulesConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PunctuationConfig {
    pub enabled: bool,
    pub standardize_dashes: bool,
    pub standardize_ellipsis: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RulesConfig {
    pub smart_quotes: SmartQuotesConfig,
    pub headings: HeadingsConfig,
    pub remove_horizontal_rules: HorizontalRulesConfig,
    pub punctuation: PunctuationConfig,
}
