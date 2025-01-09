// crates/simora_configuration/src/global_config.rs

use std::fs;
use serde::{Deserialize, Serialize}; // Add Serialize and Deserialize imports
use crate::types::{
    MarkdownFormatterConfig, PartialFilesConfiguration,
    PartialVcsConfiguration,
};

// Define a struct for the global configuration
#[derive(Serialize, Deserialize)] // Derive Serialize and Deserialize
pub struct GlobalConfig {
    pub markdown: Option<MarkdownFormatterConfig>,
    pub files: Option<PartialFilesConfiguration>,
    pub vcs: Option<PartialVcsConfiguration>,
}

// Global variable to hold the resolved configuration
pub static mut GLOBAL_CONFIG: Option<GlobalConfig> = None;

/// Initializes the global configuration
pub fn initialize_global_config(config: GlobalConfig) {
    unsafe {
        GLOBAL_CONFIG = Some(config);
    }
}

/// Gets a reference to the global configuration
pub fn get_global_config() -> Option<&'static GlobalConfig> {
    unsafe { GLOBAL_CONFIG.as_ref() }
}

/// Gets the markdown formatter configuration from global config
pub fn get_markdown_config() -> Option<&'static MarkdownFormatterConfig> {
    get_global_config().and_then(|config| config.markdown.as_ref())
}
