use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "axtc", about = "Arch/X theme changer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Apply a theme by name or path
    Apply {
        /// Name of the theme (must exist in ~/.config/axtc/themes/)
        #[arg(required_unless_present = "file", conflicts_with = "file")]
        theme: Option<String>,
        /// Path to a theme TOML file
        #[arg(long, short = 'f', conflicts_with = "theme")]
        file: Option<PathBuf>,
        /// Render templates and write output to the current directory instead of the real config paths
        #[arg(long)]
        dry_run: bool,
    },
    /// List available themes
    List,
    /// Create a new theme file from the base template
    New {
        /// Name for the new theme
        name: String,
    },
}
