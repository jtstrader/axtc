//! AXTC functionality.

use std::fmt::Display;

use clap::ValueEnum;
use tabled::{
    Table,
    settings::{Border, Style},
};

pub mod theme;
pub mod utils;

/// The different ways to list the stored themes.
#[derive(ValueEnum, Debug, Clone)]
pub enum ListMode {
    Saved,
    Recovered,
    All,
}

impl Display for ListMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Saved => "saved",
                Self::Recovered => "recovered",
                Self::All => "all",
            }
        )
    }
}

/// Save the current theme with the provided name.
pub fn save(new_theme_name: String) {}

/// Load the provided theme. If `unsafe_load` is `false`, save the current theme in the recovery
/// section.
pub fn load(theme_name: String, unsafe_load: bool, recovery: bool) {}

/// List the available themes.
pub fn list(mode: ListMode) {
    let list = match mode {
        ListMode::Saved => &theme::THEME_LIST.themes,
        ListMode::Recovered => &theme::THEME_LIST.recovered_themes,
        ListMode::All => &[
            &theme::THEME_LIST.themes[..],
            &theme::THEME_LIST.recovered_themes[..],
        ]
        .concat(),
    };

    let mut table_theme = tabled::settings::Theme::from_style(Style::ascii_rounded());
    table_theme.remove_borders();

    let mut theme_table = Table::new(list);
    theme_table.with(table_theme);

    println!("{}", theme_table);
}
