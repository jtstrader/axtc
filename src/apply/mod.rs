//! Applies a theme by rendering each app's template and writing the output to its config path.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

use crate::constants::{CONFIG_DIR, TEMPLATES_DIR};
use crate::theme::Theme;

/// A writeable theme that has been processed by Tera.
pub struct WriteableTheme {
    pub(crate) content: String,
    pub(crate) rel_path: PathBuf,
}

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

impl App {
    /// Get the relative path to a series of config files based on the app type.
    pub fn get_config_file_rel_paths(&self) -> Vec<PathBuf> {
        let prefix = self.to_string();
        match self {
            Self::Herbstluftwm => vec!["autostart"],
            Self::Alacritty => vec!["alacritty.toml"],
            Self::Polybar => vec!["config.ini", "launch.py", "scripts/tags.py"],
            Self::Picom => vec!["picom.conf"],
        }
        .into_iter()
        .map(|p| Path::new(&prefix).join(PathBuf::from(p)))
        .collect()
    }

    /// Attempt to render all configs associated with an application. If a theme template is missing
    /// it is silently ignored. However, if one of the templates exists and fails to render
    /// correctly an error is returned instead.
    pub fn render_theme(&self, theme: &Theme) -> Result<Vec<WriteableTheme>> {
        let found_templates = self.get_config_file_rel_paths().into_iter().flat_map(|p| {
            let t = TEMPLATES_DIR.join(&p).with_added_extension("tera");
            if !t.exists() || !t.is_file() {
                println!("[{}] template '{}' not found, skipping", *self, p.display());
                return None;
            }
            Some((p, t))
        });

        let mut files = vec![];
        for (rel_path, tpl) in found_templates {
            files.push(WriteableTheme {
                content: crate::template::render(&tpl, theme)?,
                rel_path,
            });
        }

        Ok(files)
    }
}

/// Render and write config files for all apps present in `theme`.
///
/// Apps whose section is absent from the theme are silently skipped.
/// Existing config files are backed up before being overwritten.
///
/// When in "dry run" mode, rendered output is written to the same relative
/// path under the current directory instead of the real config locations, and
/// no backups are created.
pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
    let writeable_cfgs = App::iter()
        .map(|app| (app, app.render_theme(theme)))
        .collect::<Vec<_>>();

    for (app, cfg_list) in writeable_cfgs {
        for cfg in cfg_list? {
            backup_and_write(app, &cfg.rel_path, &cfg.content, dry_run)?;
        }
    }

    Ok(())
}

fn backup_and_write(app: App, rel: &Path, content: &str, dry_run: bool) -> Result<()> {
    let dest = match dry_run {
        true => Path::new(".").join(rel),
        false => CONFIG_DIR.join(rel),
    };

    // Create backup
    if dest.exists() && !dry_run {
        let backup_dir = CONFIG_DIR
            .join("axtc")
            .join("backups")
            .join(app.to_string());
        std::fs::create_dir_all(&backup_dir)?;

        let filename = dest.file_name().unwrap().to_string_lossy();
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let backup_path = backup_dir.join(format!("{ts}_{filename}"));
        std::fs::copy(&dest, &backup_path)
            .with_context(|| format!("could not backup '{}'", dest.display()))?;
        println!(
            "[{}] backed up {} → {}",
            app,
            dest.display(),
            backup_path.display()
        );
    }

    // Write config
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&dest, content)
        .with_context(|| format!("could not write '{}'", dest.display()))?;

    let clean_path = dest.strip_prefix("./").unwrap_or(&dest);
    println!("[{}] {}", app, clean_path.display());
    Ok(())
}
