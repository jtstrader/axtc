//! Theme configuration types, mirroring the structure of a theme TOML file.

use serde::{Deserialize, Serialize};

/// Top-level theme definition loaded from a TOML file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    /// Unique theme name used to load it by key.
    pub name: String,
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Shared settings that all app configs may inherit from.
    pub global: Option<GlobalConfig>,
    /// ANSI terminal color palette.
    pub ansi: Option<AnsiConfig>,
    /// herbstluftwm window manager settings.
    pub herbstluftwm: Option<HerbstluftwmConfig>,
    /// polybar status-bar settings.
    pub polybar: Option<PolybarConfig>,
    /// Alacritty terminal emulator settings.
    pub alacritty: Option<AlacrittyConfig>,
    /// picom compositor settings.
    pub picom: Option<PicomConfig>,
}

/// Settings shared across all app configs (e.g. font family, terminal).
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    /// Font family used as the default across alacritty, polybar, and herbstluftwm.
    /// Individual app configs override this when set.
    pub font: Option<String>,
}

/// ANSI terminal color palette, split into primary, normal, and bright groups.
#[derive(Debug, Deserialize, Serialize)]
pub struct AnsiConfig {
    /// Background, foreground, and cursor colors.
    pub primary: Option<PrimaryColors>,
    /// Standard 8 ANSI colors (indices 0–7).
    pub normal: Option<NormalColors>,
    /// Bright variants of the 8 ANSI colors (indices 8–15).
    pub bright: Option<BrightColors>,
}

/// Background, foreground, and cursor colors for the terminal.
#[derive(Debug, Deserialize, Serialize)]
pub struct PrimaryColors {
    /// Default text color.
    pub foreground: Option<String>,
    /// Terminal background color.
    pub background: Option<String>,
    /// Cursor color.
    pub cursor: Option<String>,
}

/// Standard 8 ANSI colors (indices 0–7). All values are hex color strings (e.g. `"#ff0000"`).
#[derive(Debug, Deserialize, Serialize)]
pub struct NormalColors {
    /// Black (ANSI 0).
    pub black: Option<String>,
    /// Red (ANSI 1).
    pub red: Option<String>,
    /// Green (ANSI 2).
    pub green: Option<String>,
    /// Yellow (ANSI 3).
    pub yellow: Option<String>,
    /// Blue (ANSI 4).
    pub blue: Option<String>,
    /// Magenta (ANSI 5).
    pub magenta: Option<String>,
    /// Cyan (ANSI 6).
    pub cyan: Option<String>,
    /// White (ANSI 7).
    pub white: Option<String>,
}

/// Bright variants of the 8 ANSI colors (indices 8–15). All values are hex color strings.
#[derive(Debug, Deserialize, Serialize)]
pub struct BrightColors {
    /// Bright black (ANSI 8).
    pub black: Option<String>,
    /// Bright red (ANSI 9).
    pub red: Option<String>,
    /// Bright green (ANSI 10).
    pub green: Option<String>,
    /// Bright yellow (ANSI 11).
    pub yellow: Option<String>,
    /// Bright blue (ANSI 12).
    pub blue: Option<String>,
    /// Bright magenta (ANSI 13).
    pub magenta: Option<String>,
    /// Bright cyan (ANSI 14).
    pub cyan: Option<String>,
    /// Bright white (ANSI 15).
    pub white: Option<String>,
}

/// herbstluftwm window manager configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct HerbstluftwmConfig {
    /// Whether window and frame borders are rendered.
    pub borders: Option<bool>,
    /// Whether per-window transparency is enabled (requires picom).
    pub transparency: Option<bool>,
    /// Path to a wallpaper image passed to `nitrogen --set-zoom-fill`.
    /// When absent, `nitrogen --restore` is used instead.
    pub background_src: Option<String>,
    /// Terminal emulator spawned by the keybind. Defaults to `"alacritty"`.
    pub terminal: Option<String>,
}

/// polybar status-bar configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct PolybarConfig {
    /// Bar position: `"top"` or `"bottom"`.
    pub position: Option<String>,
    /// Bar height in points.
    pub height: Option<u32>,
    /// Full polybar font string (e.g. `"firacode:size=12;2"`).
    /// Falls back to `global.font` and then a hardcoded default when absent.
    pub font: Option<String>,
    /// Accent color used for module labels and icons (hex string).
    /// Defaults to `#ff8da1` when absent.
    pub primary_color: Option<String>,
    /// Background color for the active workspace label (hex string).
    /// Defaults to `#2b1046` when absent.
    pub background_alt: Option<String>,
}

/// Alacritty terminal emulator configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct AlacrittyConfig {
    /// Font family name (e.g. `"FiraCode"`).
    /// Falls back to `global.font` when absent.
    pub font: Option<String>,
    /// Font size in points.
    pub font_size: Option<f32>,
    /// Font style (e.g. `"SemiBold"`). Defaults to `"SemiBold"` when absent.
    pub font_style: Option<String>,
    /// Window background opacity in the range `0.0`–`1.0`.
    pub opacity: Option<f32>,
}

/// picom compositor configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct PicomConfig {
    /// Enable per-window inactive opacity (sets `inactive-opacity = 0.9`).
    pub transparency: Option<bool>,
    /// Enable background blur using the `dual_kawase` method.
    pub blur: Option<bool>,
    /// Window corner radius in pixels. `0` disables rounded corners.
    pub corner_radius: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_theme() {
        let toml = r#"name = "test""#;
        let theme: Theme = toml::from_str(toml).unwrap();
        assert_eq!(theme.name, "test");
        assert!(theme.ansi.is_none());
    }

    #[test]
    fn parse_full_theme() {
        let toml = r##"
name = "full"
description = "A full theme"

[ansi.primary]
foreground = "#e0e0e0"
background = "#000000"

[ansi.normal]
black = "#000000"
red   = "#cc0000"
green = "#4e9a06"

[herbstluftwm]
borders = true
transparency = false

[picom]
corner_radius = 8
blur = false
"##;
        let theme: Theme = toml::from_str(toml).unwrap();
        assert_eq!(theme.name, "full");
        let ansi = theme.ansi.unwrap();
        let primary = ansi.primary.unwrap();
        assert_eq!(primary.foreground.unwrap(), "#e0e0e0");
        let hlwm = theme.herbstluftwm.unwrap();
        assert_eq!(hlwm.borders.unwrap(), true);
    }

    #[test]
    fn round_trip_serialize() {
        let toml = r#"
name = "round-trip"

[alacritty]
font_size = 12.0
opacity = 1.0
"#;
        let theme: Theme = toml::from_str(toml).unwrap();
        let serialized = toml::to_string(&theme).unwrap();
        let reparsed: Theme = toml::from_str(&serialized).unwrap();
        assert_eq!(reparsed.name, "round-trip");
        let alacritty = reparsed.alacritty.unwrap();
        assert_eq!(alacritty.font_size.unwrap(), 12.0);
    }
}
