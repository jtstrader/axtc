use std::{fmt::Display, fs, io::Write, sync::LazyLock};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use utils::constants;

use crate::utils;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ThemeList {
    pub themes: Vec<Theme>,
    pub recovered_themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize, Tabled, Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub description: ThemeDescription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDescription(pub Option<String>);

impl Display for ThemeDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "N/A"),
        }
    }
}

pub static THEME_LIST: LazyLock<ThemeList> = LazyLock::new(|| {
    let theme_list_path = constants::AXTC_CONFIG_DIR.join(constants::THEME_LIST_FILENAME);
    if theme_list_path.exists() && theme_list_path.is_file() {
        let json = fs::read_to_string(&theme_list_path)
            .expect("theme list exists but file could not be opened");
        println!("json: {}", json);
        return serde_json::from_str::<ThemeList>(&json).expect("failed to deserialize theme list");
    }

    // Theme list does not exist.
    let tl = ThemeList::default();
    fs::create_dir_all(&*constants::AXTC_CONFIG_DIR).expect("could not create axtc config");
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&theme_list_path)
        .expect("could not create theme list file");
    let json = serde_json::to_string(&tl).expect("could not serialize empty theme list");
    f.write_all(json.as_bytes())
        .expect("could not write new theme list");
    tl
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_list_serialization() {
        let tl = ThemeList {
            themes: vec![Theme {
                name: "gruvbox".into(),
                description: ThemeDescription(Some("pretty!".into())),
            }],

            recovered_themes: vec![Theme {
                name: "tmp-theme".into(),
                description: ThemeDescription(None),
            }],
        };

        let json = serde_json::to_string(&tl).unwrap();
        assert_eq!(
            &json,
            "{\"themes\":[{\"name\":\"gruvbox\",\"description\":\"pretty!\"}],\"recovered_themes\":[{\"name\":\"tmp-theme\",\"description\":null}]}"
        );
    }

    #[test]
    fn theme_list_deserialization() {
        let json = "{\"themes\":[{\"name\":\"gruvbox\",\"description\":\"pretty!\"}],\"recovered_themes\":[{\"name\":\"tmp-theme\",\"description\":null}]}";
        let tl = serde_json::from_str::<ThemeList>(json).unwrap();

        // Themes
        assert!(tl.themes.len() == 1);
        assert_eq!(tl.themes[0].name, "gruvbox");
        assert_eq!(tl.themes[0].description.0.as_ref().unwrap(), "pretty!");

        // Recovered themes
        assert!(tl.recovered_themes.len() == 1);
        assert_eq!(tl.recovered_themes[0].name, "tmp-theme");
        assert!(tl.recovered_themes[0].description.0.is_none());
    }

    #[test]
    fn theme_list_serde_back_and_forth() {
        let tl = ThemeList {
            themes: vec![Theme {
                name: "gruvbox".into(),
                description: ThemeDescription(Some("pretty!".into())),
            }],

            recovered_themes: vec![Theme {
                name: "tmp-theme".into(),
                description: ThemeDescription(None),
            }],
        };

        let json = serde_json::to_string(&tl).unwrap();
        let tl = serde_json::from_str::<ThemeList>(&json).unwrap();

        // Themes
        assert!(tl.themes.len() == 1);
        assert_eq!(tl.themes[0].name, "gruvbox");
        assert_eq!(tl.themes[0].description.0.as_ref().unwrap(), "pretty!");

        // Recovered themes
        assert!(tl.recovered_themes.len() == 1);
        assert_eq!(tl.recovered_themes[0].name, "tmp-theme");
        assert!(tl.recovered_themes[0].description.0.is_none());
    }
}
