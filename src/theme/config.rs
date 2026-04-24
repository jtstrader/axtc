//! Theme configuration types, mirroring the structure of a theme TOML file.
//!
//! Each struct corresponds to one TOML section. All fields are optional at the
//! TOML level (represented as `Option<_>`) — the application is silently
//! skipped if its top-level section is absent from the file.
//!
//! | TOML section       | Struct                  | Tera root path      |
//! |--------------------|-------------------------|---------------------|
//! | `[global]`         | [`GlobalConfig`]        | `global.*`          |
//! | `[ansi.*]`         | [`AnsiConfig`]          | `ansi.*`            |
//! | `[herbstluftwm]`   | [`HerbstluftwmConfig`]  | `herbstluftwm.*`    |
//! | `[polybar]`        | [`PolybarConfig`]       | `polybar.*`         |
//! | `[alacritty]`      | [`AlacrittyConfig`]     | `alacritty.*`       |
//! | `[picom]`          | [`PicomConfig`]         | `picom.*`           |

use serde::{Deserialize, Serialize};

/// Top-level theme definition loaded from a TOML file.
///
/// # TOML
///
/// ```toml
/// name        = "my-theme"
/// description = "Optional description"
///
/// [global]      # → global.*
/// [ansi.primary]
/// [ansi.normal]
/// [ansi.bright]
/// [herbstluftwm]
/// [polybar]
/// [alacritty]
/// [picom]
/// ```
///
/// Only `name` is required. Any section may be omitted; the corresponding
/// application is silently skipped during `axtc apply`.
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
///
/// # TOML
///
/// ```toml
/// [global]
/// font = "FiraCode"
/// ```
///
/// # Tera
///
/// ```text
/// {% if global and global.font %}
/// font-family = "{{ global.font }}"
/// {% endif %}
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfig {
    /// Font family used as the default across alacritty, polybar, and herbstluftwm.
    /// Individual app configs override this when set.
    pub font: Option<String>,
}

/// ANSI terminal color palette, split into primary, normal, and bright groups.
///
/// # TOML
///
/// ```toml
/// [ansi.primary]
/// foreground = "#e0e0e0"
/// background = "#1a1a2e"
/// cursor     = "#ffffff"
///
/// [ansi.normal]
/// black = "#0d0d0d"
/// red   = "#e06c75"
/// # … green, yellow, blue, magenta, cyan, white
///
/// [ansi.bright]
/// black = "#5c6370"
/// # … same keys as [ansi.normal]
/// ```
///
/// # Tera
///
/// ```text
/// {% if ansi and ansi.primary %}
/// background = "{{ ansi.primary.background | default(value="#1a1a2e") }}"
/// foreground = "{{ ansi.primary.foreground | default(value="#e0e0e0") }}"
/// {% endif %}
///
/// {% if ansi and ansi.normal %}
/// red  = "{{ ansi.normal.red  | default(value="#e06c75") }}"
/// blue = "{{ ansi.normal.blue | default(value="#61afef") }}"
/// {% endif %}
/// ```
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
///
/// # TOML
///
/// ```toml
/// [herbstluftwm]
/// borders        = true
/// transparency   = false
/// terminal       = "alacritty"
/// background_src = "~/Pictures/Wallpapers/forest.png"
/// ```
///
/// # Tera
///
/// ```text
/// {% if herbstluftwm.borders %}
/// hc set window_border_width 2
/// {% endif %}
///
/// {% if herbstluftwm.background_src %}
/// nitrogen --set-zoom-fill "{{ herbstluftwm.background_src }}"
/// {% else %}
/// nitrogen --restore
/// {% endif %}
///
/// hc spawn {{ herbstluftwm.terminal | default(value="alacritty") }}
/// ```
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
///
/// # TOML
///
/// ```toml
/// [polybar]
/// position         = "top"
/// height           = 24
/// font             = "firacode:fontformat=truetype:style=Semibold:size=12;2"
/// primary_color    = "#ff8da1"
/// background_alt   = "#2b1046"
/// ```
///
/// # Tera
///
/// ```text
/// [bar/bar]
/// bottom = {{ polybar.position | default(value="top") == "bottom" }}
/// height = {{ polybar.height   | default(value=24) }}
///
/// {% if polybar.font %}
/// font-0 = "{{ polybar.font }}"
/// {% elif global and global.font %}
/// font-0 = "{{ global.font }}:size=12;2"
/// {% endif %}
///
/// background = {{ ansi.primary.background | default(value="#1a1a2e") }}
/// foreground = {{ ansi.primary.foreground | default(value="#e0e0e0") }}
/// ```
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
///
/// # TOML
///
/// ```toml
/// [alacritty]
/// font       = "FiraCode"
/// font_style = "SemiBold"
/// font_size  = 12.0
/// opacity    = 1.0
/// ```
///
/// # Tera
///
/// ```text
/// [font]
/// size = {{ alacritty.font_size | default(value=12.0) }}
///
/// {# app font takes priority over global font #}
/// {% if alacritty and alacritty.font %}
/// normal = { family = "{{ alacritty.font }}", style = "{{ alacritty.font_style | default(value="SemiBold") }}" }
/// {% elif global and global.font %}
/// normal = { family = "{{ global.font }}",    style = "{{ alacritty.font_style | default(value="SemiBold") }}" }
/// {% endif %}
///
/// [window]
/// opacity = {{ alacritty.opacity | default(value=1.0) }}
/// ```
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
///
/// # TOML
///
/// ```toml
/// [picom]
/// transparency  = false
/// blur          = true
/// corner_radius = 8
/// ```
///
/// # Tera
///
/// ```text
/// corner-radius = {{ picom.corner_radius | default(value=0) }};
///
/// {% if picom.transparency %}
/// inactive-opacity = 0.9;
/// {% else %}
/// inactive-opacity = 1.0;
/// {% endif %}
///
/// {% if picom.blur %}
/// blur-method = "dual_kawase";
/// blur-size    = 12;
/// {% endif %}
/// ```
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
        assert!(hlwm.borders.unwrap());
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
