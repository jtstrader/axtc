use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
    if theme.alacritty.is_none() {
        return Ok(());
    }
    let template = match super::template_path("alacritty", "alacritty.toml.tera") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("[alacritty] template not found, skipping");
            return Ok(());
        }
    };
    let rendered = crate::template::render(&template, theme)?;
    let target = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?
        .join("alacritty")
        .join("alacritty.toml");
    super::backup_and_write(&target, &rendered, "alacritty", dry_run)?;
    println!("[alacritty] applied");
    Ok(())
}
