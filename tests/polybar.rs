use std::path::{Path, PathBuf};

use axtc::template::render;
use axtc::theme::{
    AnsiConfig, BrightColors, GlobalConfig, NormalColors, PolybarConfig, PrimaryColors, Theme,
};

fn template_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/polybar/config.ini.tera")
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

// ── Bar section ───────────────────────────────────────────────────────────────

#[test]
fn bar_section_is_bar_not_main() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("[bar/bar]"));
    assert!(!out.contains("[bar/main]"));
}

#[test]
fn radius_always_present() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("radius  = 10"));
}

// ── Colors block ──────────────────────────────────────────────────────────────

#[test]
fn ansi_primary_used_for_background_and_foreground() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: Some(PrimaryColors {
            background: Some("#11061c".into()),
            foreground: Some("#dddddd".into()),
            cursor: None,
        }),
        normal: None,
        bright: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("background     = #11061c"));
    assert!(out.contains("foreground     = #dddddd"));
}

#[test]
fn default_colors_when_ansi_absent() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("background     = #1a1a2e"));
    assert!(out.contains("foreground     = #e0e0e0"));
    assert!(out.contains("primary        = #ff8da1"));
    assert!(out.contains("disabled       = #707880"));
}

#[test]
fn custom_primary_color() {
    let mut theme = minimal_theme();
    theme.polybar = Some(PolybarConfig {
        position: None,
        height: None,
        font: None,
        primary_color: Some("#aabbcc".into()),
        background_alt: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("primary        = #aabbcc"));
}

#[test]
fn custom_background_alt() {
    let mut theme = minimal_theme();
    theme.polybar = Some(PolybarConfig {
        position: None,
        height: None,
        font: None,
        primary_color: None,
        background_alt: Some("#deadbe".into()),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("background-alt = #deadbe"));
}

#[test]
fn ansi_normal_cyan_used_for_secondary() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: None,
        normal: Some(NormalColors {
            black: None,
            red: None,
            green: None,
            yellow: None,
            blue: None,
            magenta: None,
            cyan: Some("#12abcd".into()),
            white: None,
        }),
        bright: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("secondary      = #12abcd"));
}

#[test]
fn ansi_bright_black_used_for_disabled() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: None,
        normal: None,
        bright: Some(BrightColors {
            black: Some("#abcdef".into()),
            red: None,
            green: None,
            yellow: None,
            blue: None,
            magenta: None,
            cyan: None,
            white: None,
        }),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("disabled       = #abcdef"));
}

// ── Font ──────────────────────────────────────────────────────────────────────

#[test]
fn app_font_takes_priority_over_global() {
    let mut theme = minimal_theme();
    theme.global = Some(GlobalConfig {
        font: Some("GlobalFont".into()),
    });
    theme.polybar = Some(PolybarConfig {
        position: None,
        height: None,
        font: Some("app-font:size=10".into()),
        primary_color: None,
        background_alt: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("font-0 = app-font:size=10"));
    assert!(!out.contains("GlobalFont"));
}

#[test]
fn falls_back_to_global_font() {
    let mut theme = minimal_theme();
    theme.global = Some(GlobalConfig {
        font: Some("MyFont".into()),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("font-0 = MyFont:fontformat=truetype:style=Semibold:size=12;2"));
}

#[test]
fn falls_back_to_hardcoded_default_font() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("font-0 = firacode:fontformat=truetype:style=Semibold:size=12;2"));
}

// ── Modules ───────────────────────────────────────────────────────────────────

#[test]
fn all_module_sections_present() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    for section in &[
        "[module/xworkspaces]",
        "[module/xwindow]",
        "[module/filesystem]",
        "[module/pulseaudio]",
        "[module/memory]",
        "[module/cpu]",
        "[module/eth]",
        "[module/date]",
    ] {
        assert!(out.contains(section), "missing section: {section}");
    }
}

#[test]
fn modules_listed_in_bar() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("modules-left  = xworkspaces xwindow"));
    assert!(out.contains("modules-right = filesystem pulseaudio memory cpu eth date"));
}
