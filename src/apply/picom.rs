use std::path::Path;

use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
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
    super::backup_and_write(Path::new("picom/picom.conf"), &rendered, dry_run)?;
    println!("[picom] applied");
    Ok(())
}
