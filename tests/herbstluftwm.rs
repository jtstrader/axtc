use std::path::{Path, PathBuf};

use axtc::template::render;
use axtc::theme::{
    AnsiConfig, GlobalConfig, HerbstluftwmConfig, NormalColors, PrimaryColors, Theme,
};

fn template_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/herbstluftwm/autostart.tera")
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

fn minimal_hlwm() -> HerbstluftwmConfig {
    HerbstluftwmConfig {
        borders: None,
        transparency: None,
        background_src: None,
        terminal: None,
    }
}

// ── Modifier key ──────────────────────────────────────────────────────────────

#[test]
fn uses_mod1_modifier() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("Mod=Mod1"));
}

// ── Terminal ──────────────────────────────────────────────────────────────────

#[test]
fn default_terminal_is_alacritty() {
    let mut theme = minimal_theme();
    theme.herbstluftwm = Some(minimal_hlwm());
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("spawn \"alacritty\""));
}

#[test]
fn custom_terminal_used_when_set() {
    let mut theme = minimal_theme();
    theme.herbstluftwm = Some(HerbstluftwmConfig {
        terminal: Some("kitty".into()),
        ..minimal_hlwm()
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("spawn \"kitty\""));
    assert!(!out.contains("spawn \"alacritty\""));
}

// ── Wallpaper ─────────────────────────────────────────────────────────────────

#[test]
fn nitrogen_restore_when_no_background_src() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("nitrogen --restore"));
    assert!(!out.contains("--set-zoom-fill"));
}

#[test]
fn nitrogen_set_zoom_fill_when_background_src_given() {
    let mut theme = minimal_theme();
    theme.herbstluftwm = Some(HerbstluftwmConfig {
        background_src: Some("~/Pictures/wall.png".into()),
        ..minimal_hlwm()
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("nitrogen --set-zoom-fill '~/Pictures/wall.png'"));
    assert!(!out.contains("nitrogen --restore"));
}

// ── Font ──────────────────────────────────────────────────────────────────────

#[test]
fn global_font_used_in_dmenu() {
    let mut theme = minimal_theme();
    theme.global = Some(GlobalConfig {
        font: Some("JetBrainsMono".into()),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("JetBrainsMono:fontformat=truetype:style=Semibold:size=16"));
}

#[test]
fn global_font_used_in_title_font() {
    let mut theme = minimal_theme();
    theme.global = Some(GlobalConfig {
        font: Some("JetBrainsMono".into()),
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("JetBrainsMono:pixelsize=13:style=semibold"));
}

#[test]
fn default_font_in_dmenu_when_global_unset() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("firacode:fontformat=truetype:style=Semibold:size=16"));
}

// ── ANSI colors ───────────────────────────────────────────────────────────────

#[test]
fn ansi_background_in_xsetroot() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: Some(PrimaryColors {
            background: Some("#1a1b26".into()),
            foreground: None,
            cursor: None,
        }),
        normal: None,
        bright: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("xsetroot -solid '#1a1b26'"));
}

#[test]
fn ansi_magenta_in_frame_active_color() {
    let mut theme = minimal_theme();
    theme.ansi = Some(AnsiConfig {
        primary: None,
        normal: Some(NormalColors {
            black: None,
            red: None,
            green: None,
            yellow: None,
            blue: None,
            magenta: Some("#bd93f9".into()),
            cyan: None,
            white: None,
        }),
        bright: None,
    });
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("#bd93f9"));
}

// ── Startup applications ──────────────────────────────────────────────────────

#[test]
fn picom_always_spawned() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("hc spawn picom"));
}

#[test]
fn panel_sh_always_launched() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("python3 ~/.config/polybar/launch.py"));
}
