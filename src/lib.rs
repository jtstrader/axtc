//! AXTC functionality.

pub mod theme;
pub mod utils;

/// Save the current theme with the provided name.
pub fn save(new_theme_name: String) {}

/// Load the provided theme. If `unsafe_load` is `false`, save the current theme in the recovery
/// section.
pub fn load(theme_name: String, unsafe_load: bool, recovery: bool) {}

/// List the available themes.
pub fn list(recovery: bool) {}
