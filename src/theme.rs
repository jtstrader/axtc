use std::{fs, sync::LazyLock};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use utils::constants;

use crate::utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeList(Vec<Theme>);

#[derive(Serialize, Deserialize, Tabled, Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub description: String,
}

pub static THEME_LIST: LazyLock<ThemeList> = LazyLock::new(|| {
    let theme_list_path = constants::AXTC_CONFIG_DIR.join(constants::THEME_LIST_FILENAME);
    if theme_list_path.exists() && theme_list_path.is_file() {
        let mut f = fs::File::open(&theme_list_path)
            .expect("theme list exists but file could not be opened");
        serde_json::from_reader(&mut f).expect("failed to deserialize theme list")
    }

    // Theme list does not exist.
    let tl = ThemeList(Vec::new());
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&theme_list_path)
        .expect("could not create theme list file");
    serde_json::to_writer_pretty(&mut f, &tl).expect("could not write new theme list");
    tl
});
