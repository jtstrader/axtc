use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "axtc", about = "Arch/X theme changer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Apply a theme by name
    Apply {
        /// Name of the theme (must exist in ~/.config/axtc/themes/)
        theme: String,
    },
    /// List available themes
    List,
    /// Create a new theme file from the base template
    New {
        /// Name for the new theme
        name: String,
    },
}
