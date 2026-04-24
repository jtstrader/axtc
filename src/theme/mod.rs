//! Theme loading and directory resolution.

pub mod config;
pub use config::{
    AlacrittyConfig, AnsiConfig, BrightColors, GlobalConfig, HerbstluftwmConfig, NormalColors,
    PicomConfig, PolybarConfig, PrimaryColors, Theme,
};

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Returns the path to the user's axtc themes directory (`$XDG_CONFIG_HOME/axtc/themes`).
pub fn themes_dir() -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("could not determine config directory")?
        .join("axtc")
        .join("themes"))
}

impl Theme {
    /// Load and deserialize a theme by name from the user's themes directory.
    pub fn load(name: &str) -> Result<Self> {
        let path = themes_dir()?.join(format!("{name}.toml"));
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("theme '{name}' not found at {}", path.display()))?;
        toml::from_str(&content).with_context(|| format!("failed to parse theme '{name}'"))
    }
}
