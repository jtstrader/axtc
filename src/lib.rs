pub mod errors;

use std::ffi::OsStr;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;

use crate::errors::AxtcError;

/// Different programs that can have their color scheme changed
#[derive(Clone, PartialEq, Debug)]
pub enum AxtcTarget {
    Herbstluftwm(PathBuf),
    Polybar(PathBuf),
    Neovim(PathBuf),
    Alacritty(PathBuf),
}

/// Data structure for maintaining all colors
#[derive(Deserialize, Debug)]
pub struct ColorScheme<'a> {
    theme: &'a str,
    color: Vec<&'a str>,
    background: &'a str,
    foreground: &'a str,
}

/// Check if path is valid and points to a JSON file
pub fn verify_input_file(path: impl Into<PathBuf>) -> Result<(), AxtcError> {
    let path: PathBuf = path.into();

    if !path.exists() {
        Err(AxtcError::FileNotFound)
    } else if path.is_dir() || path.extension().unwrap_or_default() != ".json" {
        Err(AxtcError::InvalidFileFormat)
    } else {
        Ok(())
    }
}

/// Write out color information to the files provided
pub fn write_colors(color_file_path: impl Into<PathBuf>, targets: &[AxtcTarget]) {
    // Deserialize data into our ColorScheme struct
    let path = color_file_path.into();
    let data = &fs::read_to_string(&path).unwrap();
    let color_scheme = match serde_json::from_str::<ColorScheme>(data) {
        Ok(cs) => {
            if cs.color.len() != 16 {
                panic!(
                    "\"{:?}\" contains invalid JSON, length of color array is {}, expected {}",
                    path,
                    cs.color.len(),
                    16
                );
            }
            cs
        }
        Err(e) => {
            panic!("{}", e);
        }
    };

    //write_alacritty(&color_scheme, alc_path);
    //write_polybar(&color_scheme, ply_path);
}

/// Write out color scheme in Alacritty format
fn write_alacritty(cs: &ColorScheme, path: &str) {
    // Open file w/ create (because we will use overwrite mode)
    match File::create(path) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    };

    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut f = BufWriter::new(f);

    let colors: [&str; 8] = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ];

    // remove # from front of color code and add 0x
    let alcfmt = |s: &str| format!("0x{}", &s[1..]);

    // do not use the variable in the current scope of the fuction, as that will give the closure
    // the access to the single, mutable reference that's allowed. We instead can pass in the mutable
    // reference, treating this like a function. Only defined as a closure as its purpose is solely
    // for this function, and will be used nowhere else.
    let write_colors = |fx: &mut BufWriter<File>, bright: bool| {
        let shift = if bright { 8 } else { 0 };
        for (i, color) in colors.iter().enumerate() {
            let line = format!("    {:<10}'{}'", format!("{}:", color), cs.color[i + shift]);
            writeln!(fx, "{}", line).unwrap();
        }
        writeln!(fx).unwrap();
    };

    // header through primary colors
    writeln!(f, "# Colors ({} Theme)\n colors:", cs.theme).unwrap();
    writeln!(f, "  # Default colors").unwrap();
    writeln!(f, "  primary:").unwrap();
    writeln!(f, "    background: '{}'", alcfmt(cs.background)).unwrap();
    writeln!(f, "    foreground: '{}'", alcfmt(cs.foreground)).unwrap();
    writeln!(f).unwrap();

    // normal colors
    writeln!(f, "  # Normal colors\n  normal:").unwrap();
    write_colors(&mut f, false);

    // bright colors
    writeln!(f, "  # Bright colors\n  bright:").unwrap();
    write_colors(&mut f, true);
}

/// Write out color scheme in Polybar format
fn write_polybar(cs: &ColorScheme, path: &str) {
    // Open file w/ create (because we will use overwrite mode)
    match File::create(path) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    };

    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let mut f = BufWriter::new(f);

    let colors: [&str; 8] = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ];

    // do not use the variable in the current scope of the fuction, as that will give the closure
    // the access to the single, mutable reference that's allowed. We instead can pass in the mutable
    // reference, treating this like a function. Only defined as a closure as its purpose is solely
    // for this function, and will be used nowhere else.
    let write_colors = |fx: &mut BufWriter<File>, bright: bool| {
        let shift = if bright { 8 } else { 0 };
        for (i, color) in colors.iter().enumerate() {
            writeln!(
                fx,
                "{} = {}",
                format!("{}{}", if bright { "alt" } else { "" }, color),
                cs.color[i + shift]
            )
            .unwrap();
        }
    };

    // write color tag and background/foreground
    writeln!(f, "[color]").unwrap();
    writeln!(f, "background = {}", cs.background).unwrap();
    writeln!(f, "foreground = {}", cs.foreground).unwrap();

    // normal colors
    write_colors(&mut f, false);

    // bright colors
    write_colors(&mut f, true);
}
