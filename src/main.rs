use std::io;
use std::process::Command;

use clap::Parser;

use axtc::init_targets;
use axtc::AxtcTarget;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The input color file that contains the information in JSON.
    color_file: String,

    #[arg(long, default_value_t = false)]
    herbstluftwm: bool,

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

        let [herbstluftwm_path, polybar_path, neovim_path, alacritty_path] = [
            "~/.config/herbstluftwm/autostart",
            "~/.config/polybar/colors.ini",
            "---TODO---",
            "~/.config/alacritty/alacritty.yml",
        ]
        .map(simple_home_dir::expand_tilde)
        .map(Option::unwrap);

        let args = [self.herbstluftwm, self.polybar, self.neovim, self.alacritty];
        if args.into_iter().all(|arg| !arg) {
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
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let (color_input_file, targets) = (args.color_file.clone(), args.gen_targets());

    match axtc::verify_input_file(&color_input_file) {
        Ok(()) => {
            axtc::write_colors(color_input_file, &targets)?;
            issue_refresh();
        }
        Err(e) => {
            eprintln!("axtc: {}", e);
            std::process::exit(1);
        }
    };

    Ok(())
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
