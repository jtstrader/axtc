//! AXTC functionality.

use tabled::{
    Table,
    settings::{Border, Style},
};

pub mod theme;
pub mod utils;

/// Save the current theme with the provided name.
pub fn save(new_theme_name: String) {}

/// Load the provided theme. If `unsafe_load` is `false`, save the current theme in the recovery
/// section.
pub fn load(theme_name: String, unsafe_load: bool, recovery: bool) {}

/// List the available themes.
pub fn list(all: bool, recovery: bool) {
    let list = match (all, recovery) {
        (true, _) => &[
            &theme::THEME_LIST.themes[..],
            &theme::THEME_LIST.recovered_themes[..],
        ]
        .concat(),
        (_, true) => &theme::THEME_LIST.recovered_themes,
        _ => &theme::THEME_LIST.themes,
    };

    let mut table_theme = tabled::settings::Theme::from_style(Style::ascii_rounded());
    table_theme.remove_borders();

    let mut theme_table = Table::new(list);
    theme_table.with(table_theme);

    println!("{}", theme_table);
}
