use std::io;
use std::process::Child;
use std::process::Command;

use anyhow::Context;
use clap::Parser;

use axtc::init_targets;
use axtc::AxtcTarget;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The input color file that contains the information in JSON.
    color_file: String,

    /// Update the colorscheme of the herbstluft window manager.
    #[arg(long, default_value_t = false)]
    herbstluftwm: bool,

    /// Update the colorscheme of the polybar.
    #[arg(long, default_value_t = false)]
    polybar: bool,

    /// Update the colorscheme of the Neovim editor.
    #[arg(long, default_value_t = false)]
    neovim: bool,

    /// Update the colorscheme of the Alacritty terminal.
    #[arg(long, default_value_t = false)]
    alacritty: bool,
}

impl Args {
    pub fn gen_targets(&self) -> Vec<AxtcTarget> {
        use AxtcTarget as AXT;

        let [herbstluftwm_path, polybar_path, neovim_path, alacritty_path] = [
            "~/.config/herbstluftwm/autostart",
            "~/.config/polybar/colors.ini",
            "---TODO---",
            "~/.config/alacritty/alacritty.yml",
        ]
        .map(simple_home_dir::expand_tilde)
        .map(Option::unwrap);

        if self.no_args_set() {
            return vec![
                AXT::Herbstluftwm(herbstluftwm_path),
                AXT::Polybar(polybar_path),
                AXT::Neovim(neovim_path),
                AXT::Alacritty(alacritty_path),
            ];
        }

        // Go through each possible arg
        init_targets!(
            self.herbstluftwm => AXT::Herbstluftwm(herbstluftwm_path),
            self.polybar => AXT::Polybar(polybar_path),
            self.neovim => AXT::Neovim(neovim_path),
            self.alacritty => AXT::Alacritty(alacritty_path)
        )
    }

    /// Check if no arguments are set. Return true if every argument in the arg struct is a false.
    fn no_args_set(&self) -> bool {
        !self.herbstluftwm && !self.polybar && !self.neovim && !self.alacritty
    }
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let (color_input_file, targets) = (args.color_file.clone(), args.gen_targets());

    match axtc::verify_input_file(&color_input_file) {
        Ok(()) => {
            axtc::write_colors(color_input_file, &targets)?;
            issue_refresh().into_iter().for_each(try_wait);
        }
        Err(e) => {
            eprintln!("axtc: {}", e);
            std::process::exit(1);
        }
    };

    Ok(())
}

/// Refresh Polybar and bspwm
fn issue_refresh() -> Vec<io::Result<Child>> {
    vec![
        Command::new("pkill").arg("polybar").spawn(),
        Command::new("herbstclient").arg("reload").spawn(),
    ]
}

/// Wait on child process or log error if the process could not be spawned.
fn try_wait(handle_result: io::Result<Child>) {
    match handle_result {
        Ok(mut handle) => {
            if let Err(e) = handle.wait() {
                eprintln!("{}", e);
            }
        }
        Err(e) => eprintln!("{}", e),
    };
}
