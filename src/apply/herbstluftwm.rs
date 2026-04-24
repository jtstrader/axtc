use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme) -> Result<()> {
    if theme.herbstluftwm.is_none() {
        return Ok(());
    }
    let template = match super::template_path("herbstluftwm", "autostart.tera") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("[herbstluftwm] template not found, skipping");
            return Ok(());
        }
    };
    let rendered = crate::template::render(&template, theme)?;
    let target = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?
        .join("herbstluftwm")
        .join("autostart");
    super::backup_and_write(&target, &rendered, "herbstluftwm")?;
    println!("[herbstluftwm] applied");
    Ok(())
}
