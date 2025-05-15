//! AXTC constants.

use std::{path::PathBuf, sync::LazyLock};

/// The name of the theme list stored in AXTC.
pub const THEME_LIST_FILENAME: &str = ".axtc-theme-list.json";

/// The AXTC config directory. Assumes that AXTC is being built and run on Linux.
pub static AXTC_CONFIG_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::config_dir().unwrap().join("axtc"));
