use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme) -> Result<()> {
    if theme.picom.is_none() {
        return Ok(());
    }
    let template = match super::template_path("picom", "picom.conf.tera") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("[picom] template not found, skipping");
            return Ok(());
        }
    };
    let rendered = crate::template::render(&template, theme)?;
    let target = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?
        .join("picom")
        .join("picom.conf");
    super::backup_and_write(&target, &rendered, "picom")?;
    println!("[picom] applied");
    Ok(())
}
