use std::path::Path;

use anyhow::Result;

use crate::theme::Theme;

pub fn apply(theme: &Theme, dry_run: bool) -> Result<()> {
    if theme.polybar.is_none() {
        return Ok(());
    }

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
        super::backup_and_write(Path::new("polybar/config.ini"), &rendered?, dry_run)?;
    }

    if let Some(rendered) = render("launch.py.tera") {
        super::backup_and_write(Path::new("polybar/launch.py"), &rendered?, dry_run)?;
    }

    if let Some(rendered) = render("scripts/tags.py.tera") {
        super::backup_and_write(Path::new("polybar/scripts/tags.py"), &rendered?, dry_run)?;
    }

    println!("[polybar] applied");
    Ok(())
}
