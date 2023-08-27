use clap::Parser;
use std::env;
use std::process::Command;

use axtc::AxtcTarget;

/// Reformats the Arch Linux with a provided color scheme file.
const ALC_PATH: &'static str = "/home/jtstr/.config/alacritty/colors.yml";
const PLY_PATH: &'static str = "/home/jtstr/.config/herbstluft/polybar/colors";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The input color file that contains the information in JSON.
    color_file: String,

    #[arg(long, default_value_t = false)]
    herbstluft: bool,

    #[arg(long, default_value_t = false)]
    polybar: bool,

    #[arg(long, default_value_t = false)]
    neovim: bool,

    #[arg(long, default_value_t = false)]
    alacritty: bool,
}

impl Args {
    pub fn gen_targets(&self) -> Vec<AxtcTarget> {
        use AxtcTarget as AXT;

        let args = [self.herbstluft, self.polybar, self.neovim, self.alacritty];
        if args.into_iter().all(|arg| !arg) {
            return vec![AXT::Herbstluftwm, AXT::Polybar, AXT::Neovim, AXT::Alacritty];
        }

        // Go through each possible arg
        let mut targets: Vec<AxtcTarget> = Vec::new();

        if self.herbstluft {
            targets.push(AXT::Herbstluftwm);
        }

        if self.polybar {
            targets.push(AXT::Polybar);
        }

        if self.neovim {
            targets.push(AXT::Neovim);
        }

        if self.alacritty {
            targets.push(AXT::Alacritty);
        }

        targets
    }
}

fn main() {
    let args: Args = Args::parse();
    let (color_input_file, targets) = (args.color_file.clone(), args.gen_targets());
    axtc::verify_input_file(&color_input_file);
    axtc::write_colors(color_input_file, &targets);
    issue_refresh();
}

/// Refresh Polybar and bspwm
fn issue_refresh() {
    Command::new("pkill")
        .arg("polybar")
        .spawn()
        .expect("failed to pkill polybar");
    Command::new("herbstclient")
        .arg("reload")
        .spawn()
        .expect("failed to reload herbstluftwm");
}
