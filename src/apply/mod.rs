//! Applies a theme by rendering each app's template and writing the output to its config path.

mod alacritty;
mod herbstluftwm;
mod picom;
mod polybar;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::theme::Theme;

/// Render and write config files for all apps present in `theme`.
///
/// Apps whose section is absent from the theme are silently skipped.
/// Existing config files are backed up before being overwritten.
pub fn apply(theme: &Theme) -> Result<()> {
    println!("Applying theme '{}'...", theme.name);
    herbstluftwm::apply(theme)?;
    polybar::apply(theme)?;
    alacritty::apply(theme)?;
    picom::apply(theme)?;
    println!("Done.");
    Ok(())
}

fn template_path(app: &str, filename: &str) -> Result<PathBuf> {
    let path = dirs::config_dir()
        .context("could not determine config directory")?
        .join("axtc")
        .join("templates")
        .join(app)
        .join(filename);
    anyhow::ensure!(path.exists(), "template not found: {}", path.display());
    Ok(path)
}

fn backup_and_write(target: &Path, content: &str, app: &str) -> Result<()> {
    if target.exists() {
        let backup_dir = dirs::config_dir()
            .context("could not determine config directory")?
            .join("axtc")
            .join("backups")
            .join(app);
        std::fs::create_dir_all(&backup_dir)?;

        let filename = target.file_name().unwrap().to_string_lossy();
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let backup_path = backup_dir.join(format!("{ts}_{filename}"));
        std::fs::copy(target, &backup_path)
            .with_context(|| format!("could not backup '{}'", target.display()))?;
        println!(
            "  backed up {} → {}",
            target.display(),
            backup_path.display()
        );
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(target, content)
        .with_context(|| format!("could not write '{}'", target.display()))?;
    println!("  wrote {}", target.display());
    Ok(())
}
