use std::path::Path;

use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
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
    super::backup_and_write(Path::new("herbstluftwm/autostart"), &rendered, dry_run)?;
    println!("[herbstluftwm] applied");
    Ok(())
}
