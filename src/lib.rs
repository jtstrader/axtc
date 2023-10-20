pub mod errors;
mod utils;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{self, BufWriter};
use std::path::PathBuf;

use serde::Deserialize;

use crate::errors::AxtcError;

const COLOR_ARRAY_LEN: usize = 16;

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
    _theme: &'a str,
    color: Vec<&'a str>,
    background: &'a str,
    foreground: &'a str,
}

/// Check if path is valid and points to a JSON file
pub fn verify_input_file(path: impl Into<PathBuf>) -> Result<(), AxtcError> {
    let path: PathBuf = path.into();

    if !path.exists() {
        Err(AxtcError::FileNotFound)
    } else if path.is_dir() || path.extension().unwrap_or_default() != "json" {
        Err(AxtcError::InvalidFileFormat)
    } else {
        Ok(())
    }
}

/// Write out color information to the files provided
pub fn write_colors(color_file_path: impl Into<PathBuf>, targets: &[AxtcTarget]) -> io::Result<()> {
    // Deserialize data into our ColorScheme struct
    let path = color_file_path.into();
    let data = &fs::read_to_string(&path).unwrap();
    let color_scheme = match serde_json::from_str::<ColorScheme>(data) {
        Ok(cs) => {
            if cs.color.len() != COLOR_ARRAY_LEN {
                panic!(
                    "\"{:?}\" contains invalid JSON, length of color array is {}, expected {}",
                    path,
                    cs.color.len(),
                    COLOR_ARRAY_LEN
                );
            }
            cs
        }
        Err(e) => {
            panic!("{}", e);
        }
    };

    for target in targets {
        match target {
            AxtcTarget::Alacritty(path) => write_alacritty(&color_scheme, path)?,
            _ => todo!(),
        };
    }

    Ok(())
}

/// Write out color scheme in Alacritty format
fn write_alacritty(cs: &ColorScheme, path: &PathBuf) -> io::Result<()> {
    // Read in contents first, only want to overrite color data
    let file_contents = fs::read_to_string(path)?;

    // Wipe file
    fs::File::create(path)?;
    let f = OpenOptions::new().write(true).open(path)?;
    let mut f = BufWriter::new(f);

    let colors: [&str; 8] = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ];

    let write_colors = |fx: &mut BufWriter<File>, bright: bool| -> io::Result<()> {
        let shift = if bright { 8 } else { 0 };
        for (i, color) in colors.iter().enumerate() {
            let line = format!("    {:<10}'{}'", format!("{}:", color), cs.color[i + shift]);
            writeln!(fx, "{}", line)?;
        }
        writeln!(fx)?;
        Ok(())
    };

    // Write out contents of config until "colors:" is reached, then begin overwriting with new
    // color config. Continue writing any additional config that occurs after.
    let lines: Vec<&str> = file_contents
        .split('\n')
        .map(|line| line.trim_end())
        .collect();
    let mut idx: usize = 0;
    while idx < lines.len() {
        // If "colors:" is detected, write out colors and iterate.
        // Otherwise just write out the current line.
        if lines[idx] != "colors:" {
            writeln!(f, "{}", lines[idx])?;
            idx += 1;
            continue;
        }

        // Header through primary colors
        writeln!(f, "colors:")?;
        writeln!(f, "  # Default colors")?;
        writeln!(f, "  primary:")?;
        writeln!(f, "    background: '{}'", cs.background)?;
        writeln!(f, "    foreground: '{}'", cs.foreground)?;
        writeln!(f)?;

        // Normal colors
        writeln!(f, "  # Normal colors\n  normal:")?;
        write_colors(&mut f, false)?;

        // Bright colors
        writeln!(f, "  # Bright colors\n  bright:")?;
        write_colors(&mut f, true)?;

        idx += 1;
        while (lines[idx].starts_with(|c| [' ', '\n', '#'].contains(&c)) || lines[idx].is_empty())
            && idx < lines.len()
        {
            idx += 1;
        }
    }

    Ok(())
}
