//! Global path constants derived from the XDG config directory.
//!
//! All three values are initialized on first access via [`LazyLock`]. If
//! [`dirs::config_dir`] cannot resolve a config directory the application
//! panics immediately — there is no meaningful way to continue without it.

use std::path::PathBuf;
use std::sync::LazyLock;

/// Root XDG config directory (`$XDG_CONFIG_HOME`, typically `~/.config`).
pub static CONFIG_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::config_dir().expect("could not determine config directory"));

/// Directory where axtc reads Tera templates (`$XDG_CONFIG_HOME/axtc/templates`).
pub static TEMPLATES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| CONFIG_DIR.join("axtc").join("templates"));

/// Directory where axtc reads theme TOML files (`$XDG_CONFIG_HOME/axtc/themes`).
pub static THEMES_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| CONFIG_DIR.join("axtc").join("themes"));
