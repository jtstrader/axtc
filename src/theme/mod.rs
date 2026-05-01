//! Theme loading and directory resolution.

pub mod config;
pub use config::{
    AlacrittyConfig, AnsiConfig, BrightColors, GlobalConfig, HerbstluftwmConfig, NormalColors,
    PicomConfig, PolybarConfig, PrimaryColors, Theme,
};

use anyhow::{Context, Result};
use std::path::Path;

use crate::constants::THEMES_DIR;

impl Theme {
    /// Load and deserialize a theme by name from the user's themes directory.
    pub fn load(name: &str) -> Result<Self> {
        let path = THEMES_DIR.join(format!("{name}.toml"));
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("theme '{name}' not found at {}", path.display()))?;
        toml::from_str(&content).with_context(|| format!("failed to parse theme '{name}'"))
    }

    /// Load and deserialize a theme from an explicit file path.
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("could not read theme file '{}'", path.display()))?;
        toml::from_str(&content)
            .with_context(|| format!("failed to parse theme from '{}'", path.display()))
    }
}
