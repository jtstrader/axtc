use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
    if theme.polybar.is_none() {
        return Ok(());
    }

    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?;

    let render = |tpl: &str| -> Option<Result<String>> {
        match super::template_path("polybar", tpl) {
            Ok(p) => Some(crate::template::render(&p, theme)),
            Err(_) => {
                eprintln!("[polybar] template '{tpl}' not found, skipping");
                None
            }
        }
    };

    if let Some(rendered) = render("config.ini.tera") {
        let target = config_dir.join("polybar").join("config.ini");
        super::backup_and_write(&target, &rendered?, "polybar", dry_run)?;
    }

    if let Some(rendered) = render("launch.py.tera") {
        let target = config_dir.join("polybar").join("launch.py");
        super::backup_and_write(&target, &rendered?, "polybar", dry_run)?;
    }

    if let Some(rendered) = render("scripts/tags.py.tera") {
        let target = config_dir.join("polybar").join("scripts").join("tags.py");
        super::backup_and_write(&target, &rendered?, "polybar", dry_run)?;
    }

    println!("[polybar] applied");
    Ok(())
}
