//! Applies a theme by rendering each app's template and writing the output to its config path.

mod alacritty;
mod herbstluftwm;
mod picom;
mod polybar;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

use crate::theme::Theme;

/// An application managed by axtc.
#[derive(Clone, Copy, Debug, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum App {
    /// herbstluftwm window manager.
    Herbstluftwm,
    /// Polybar status bar.
    Polybar,
    /// Alacritty terminal emulator.
    Alacritty,
    /// Picom compositor.
    Picom,
}

/// Render and write config files for all apps present in `theme`.
///
/// Apps whose section is absent from the theme are silently skipped.
/// Existing config files are backed up before being overwritten.
///
/// When `dry_run` is `true`, rendered output is written to the same relative
/// path under the current directory instead of the real config locations, and
/// no backups are created.
pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
    if dry_run {
        println!(
            "Dry-run: rendering theme '{}' into current directory...",
            theme.name
        );
    } else {
        println!("Applying theme '{}'...", theme.name);
    }

    App::iter().for_each(|app| {
        if let Err(e) = apply_theme(app, theme, dry_run) {
            eprintln!("[{}] error: {e}", app);
        }
    });

    println!("Done.");
    Ok(())
}

fn apply_theme(app: App, theme: &Theme, dry_run: bool) -> Result<()> {
    match app {
        App::Herbstluftwm => herbstluftwm::apply(theme, dry_run),
        App::Polybar => polybar::apply(theme, dry_run),
        App::Alacritty => alacritty::apply(theme, dry_run),
        App::Picom => picom::apply(theme, dry_run),
    }
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

fn backup_and_write(rel: &Path, content: &str, dry_run: bool) -> Result<()> {
    if dry_run {
        let dest = Path::new(".").join(rel);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&dest, content)
            .with_context(|| format!("could not write '{}'", dest.display()))?;
        println!("  [dry-run] {}", dest.display());
        return Ok(());
    }

    let config_dir = dirs::config_dir().context("could not determine config directory")?;
    let target = config_dir.join(rel);

    if target.exists() {
        let app_name = rel
            .components()
            .next()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".to_owned());
        let backup_dir = config_dir.join("axtc").join("backups").join(&app_name);
        std::fs::create_dir_all(&backup_dir)?;

        let filename = target.file_name().unwrap().to_string_lossy();
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let backup_path = backup_dir.join(format!("{ts}_{filename}"));
        std::fs::copy(&target, &backup_path)
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
    std::fs::write(&target, content)
        .with_context(|| format!("could not write '{}'", target.display()))?;
    println!("  wrote {}", target.display());
    Ok(())
}
