use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme) -> Result<()> {
    if theme.polybar.is_none() {
        return Ok(());
    }
    let template = match super::template_path("polybar", "config.ini.tera") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("[polybar] template not found, skipping");
            return Ok(());
        }
    };
    let rendered = crate::template::render(&template, theme)?;
    let target = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?
        .join("polybar")
        .join("config.ini");
    super::backup_and_write(&target, &rendered, "polybar")?;
    println!("[polybar] applied");
    Ok(())
}
