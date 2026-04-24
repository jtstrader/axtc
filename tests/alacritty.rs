use std::path::{Path, PathBuf};

use axtc::template::render;
use axtc::theme::{
    AlacrittyConfig, AnsiConfig, BrightColors, GlobalConfig, NormalColors, PrimaryColors, Theme,
};

fn template_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/alacritty/alacritty.toml.tera")
}

fn minimal_theme() -> Theme {
    Theme {
        name: "test".into(),
        description: None,
        global: None,
        ansi: None,
        herbstluftwm: None,
        polybar: None,
        alacritty: None,
        picom: None,
    }
}

// ── Keyboard binding ───────────────────────────────────────────────────────────

#[test]
fn keyboard_binding_always_present() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("[[keyboard.bindings]]"));
    assert!(out.contains("key   = \"Return\""));
    assert!(out.contains("mods  = \"Shift\""));
}

// ── ANSI colors ───────────────────────────────────────────────────────────────

#[test]
fn ansi_primary_colors_render() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: Some(PrimaryColors {
            background: Some("#282a36".into()),
            foreground: Some("#f8f8f2".into()),
            cursor: None,
        }),
        normal: None,
        bright: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains(r##"background = "#282a36""##));
    assert!(out.contains(r##"foreground = "#f8f8f2""##));
}

#[test]
fn ansi_normal_colors_render() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: None,
        normal: Some(NormalColors {
            black: Some("#000000".into()),
            red: Some("#ff5555".into()),
            green: Some("#50fa7b".into()),
            yellow: Some("#f1fa8c".into()),
            blue: Some("#6272a4".into()),
            magenta: Some("#ff79c6".into()),
            cyan: Some("#8be9fd".into()),
            white: Some("#bfbfbf".into()),
        }),
        bright: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("[colors.normal]"));
    assert!(out.contains(r##"red     = "#ff5555""##));
    assert!(out.contains(r##"magenta = "#ff79c6""##));
}

#[test]
fn ansi_bright_colors_render() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: None,
        normal: None,
        bright: Some(BrightColors {
            black: Some("#4d4d4d".into()),
            red: Some("#ff6e6e".into()),
            green: Some("#69ff94".into()),
            yellow: Some("#ffffa5".into()),
            blue: Some("#d6acff".into()),
            magenta: Some("#ff92df".into()),
            cyan: Some("#a4ffff".into()),
            white: Some("#ffffff".into()),
        }),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("[colors.bright]"));
    assert!(out.contains(r##"magenta = "#ff92df""##));
}

// ── Font family ───────────────────────────────────────────────────────────────

#[test]
fn app_font_takes_priority_over_global() {
    let mut theme = minimal_theme();
    theme.global = Some(GlobalConfig {
        font: Some("GlobalFont".into()),
    });
    theme.alacritty = Some(AlacrittyConfig {
        font: Some("AppFont".into()),
        font_size: None,
        font_style: None,
        opacity: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("AppFont"));
    assert!(!out.contains("GlobalFont"));
}

#[test]
fn falls_back_to_global_font() {
    let mut theme = minimal_theme();
    theme.global = Some(GlobalConfig {
        font: Some("GlobalFont".into()),
    });
    theme.alacritty = Some(AlacrittyConfig {
        font: None,
        font_size: None,
        font_style: None,
        opacity: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("GlobalFont"));
}

#[test]
fn no_font_family_when_unset() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(!out.contains("family"));
}

// ── Font style ────────────────────────────────────────────────────────────────

#[test]
fn font_style_defaults_to_semibold() {
    let mut theme = minimal_theme();
    theme.alacritty = Some(AlacrittyConfig {
        font: Some("FiraCode".into()),
        font_size: None,
        font_style: None,
        opacity: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("style = \"SemiBold\""));
}

#[test]
fn custom_font_style_overrides_default() {
    let mut theme = minimal_theme();
    theme.alacritty = Some(AlacrittyConfig {
        font: Some("FiraCode".into()),
        font_size: None,
        font_style: Some("Light".into()),
        opacity: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("style = \"Light\""));
    assert!(!out.contains("SemiBold"));
}

// ── Opacity ───────────────────────────────────────────────────────────────────

#[test]
fn opacity_value_renders() {
    let mut theme = minimal_theme();
    theme.alacritty = Some(AlacrittyConfig {
        font: None,
        font_size: None,
        font_style: None,
        opacity: Some(0.5),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("[window]"));
    assert!(out.contains("opacity = 0.5"));
}

#[test]
fn opacity_defaults_to_one() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("opacity = 1"));
}
