use clap::{Parser, Subcommand};

/// All available options for AXTC.
#[derive(Subcommand, Debug, Clone)]
enum Options {
    /// Save the current theme.
    Save {
        /// The name of the new theme. Must not already exist in the theme list.
        new_name: String,
    },

    /// Load the given theme.
    Load {
        /// The name of the theme to load. It must exist in the theme list.
        theme: String,

        /// Load the theme without saving a copy of the current theme.
        #[clap(name = "unsafe", long, default_value_t = false)]
        unsafe_load: bool,

        /// Include searches in the recovery section.
        #[clap(long, default_value_t = false)]
        recovery: bool,
    },

    /// List all supported themes.
    List {
        /// Show all themes (saved and recovered). Cannot be used with the recovery flag.
        #[clap(long, default_value_t = false, conflicts_with = "recovery")]
        all: bool,

        /// Only show recovered themes in the listing. Cannot be used with the all flag.
        #[clap(long, default_value_t = false, conflicts_with = "all")]
        recovery: bool,
    },
}

/// An Arch/X theme changer.
#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    opt: Options,
}

fn main() {
    let args = Args::parse();
    match args.opt {
        Options::Save { new_name } => axtc_lib::save(new_name),
        Options::Load {
            theme,
            unsafe_load,
            recovery,
        } => axtc_lib::load(theme, unsafe_load, recovery),
        Options::List { all, recovery } => axtc_lib::list(all, recovery),
    }
}
