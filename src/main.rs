use anyhow::Result;
use clap::Parser;

mod cli;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Apply { theme, file, dry_run } => {
            let loaded = match (theme, file) {
                (Some(name), None) => axtc::theme::Theme::load(&name)?,
                (None, Some(p)) => axtc::theme::Theme::load_from_path(&p)?,
                _ => unreachable!(),
            };
            axtc::apply::apply(&loaded, dry_run)
        }
        Command::List => list(),
        Command::New { name } => new_theme(&name),
    }
}

fn list() -> Result<()> {
    let themes_dir = axtc::theme::themes_dir()?;
    if !themes_dir.exists() {
        anyhow::bail!("themes directory not found: {}", themes_dir.display());
    }
    let mut themes: Vec<String> = std::fs::read_dir(&themes_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            let stem = path.file_stem()?.to_str()?.to_owned();
            if path.extension()?.to_str()? == "toml" && stem != "template" {
                Some(stem)
            } else {
                None
            }
        })
        .collect();
    themes.sort();
    if themes.is_empty() {
        println!("No themes found in {}", themes_dir.display());
    } else {
        for t in themes {
            println!("{t}");
        }
    }
    Ok(())
}

fn new_theme(name: &str) -> Result<()> {
    let themes_dir = axtc::theme::themes_dir()?;
    let template_path = themes_dir.join("template.toml");
    let dest_path = themes_dir.join(format!("{name}.toml"));
    anyhow::ensure!(
        template_path.exists(),
        "template.toml not found in {}",
        themes_dir.display()
    );
    anyhow::ensure!(!dest_path.exists(), "theme '{name}' already exists");
    std::fs::copy(&template_path, &dest_path)?;
    println!("Created {}", dest_path.display());
    Ok(())
}
