use arch_extendable_theme_changer as axtc;

use clap::Parser;
use std::env;
use std::process::Command;

/// Reformats the Arch Linux with a provided color scheme file.
const ALC_PATH: &'static str = "/home/jtstr/.config/alacritty/colors.yml";
const PLY_PATH: &'static str = "/home/jtstr/.config/herbstluft/polybar/colors";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = false)]
    herbstluft: bool,

    #[arg(long, default_value_t = false)]
    polybar: bool,

    #[arg(long, default_value_t = false)]
    vim: bool,

    #[arg(long, default_value_t = false)]
    alacritty: bool,
}

fn main() {
    let args_list: Vec<String> = env::args().collect();
    if args_list.len() == 1 {
        panic!("no provided input file");
    }
    let color_input_file: &str = &args_list[1];

    axtc::verify_input_file(color_input_file);
    axtc::write_colors(color_input_file);
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
