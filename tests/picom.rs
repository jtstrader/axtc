use std::path::{Path, PathBuf};

use axtc::template::render;
use axtc::theme::{PicomConfig, Theme};

fn template_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/picom/picom.conf.tera")
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

fn picom(transparency: bool, blur: bool, corner_radius: u32) -> PicomConfig {
    PicomConfig {
        transparency: Some(transparency),
        blur: Some(blur),
        corner_radius: Some(corner_radius),
    }
}

// ── Backend ───────────────────────────────────────────────────────────────────

#[test]
fn backend_is_always_glx() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("backend = \"glx\";"));
}

// ── Transparency ──────────────────────────────────────────────────────────────

#[test]
fn transparency_enabled_sets_inactive_opacity() {
    let mut theme = minimal_theme();
    theme.picom = Some(picom(true, false, 0));
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("inactive-opacity = 0.9;"));
    assert!(out.contains("frame-opacity    = 0.9;"));
}

#[test]
fn transparency_disabled_sets_full_opacity() {
    let mut theme = minimal_theme();
    theme.picom = Some(picom(false, false, 0));
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("inactive-opacity = 1.0;"));
}

#[test]
fn no_picom_config_defaults_to_full_opacity() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("inactive-opacity = 1.0;"));
}

// ── Blur ──────────────────────────────────────────────────────────────────────

#[test]
fn blur_enabled_renders_blur_method() {
    let mut theme = minimal_theme();
    theme.picom = Some(picom(false, true, 0));
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("blur-method    = \"dual_kawase\";"));
    assert!(out.contains("blur-background = true;"));
}

#[test]
fn blur_disabled_omits_blur_section() {
    let mut theme = minimal_theme();
    theme.picom = Some(picom(false, false, 0));
    let out = render(&template_path(), &theme).unwrap();
    assert!(!out.contains("blur-method"));
}

// ── Corner radius ─────────────────────────────────────────────────────────────

#[test]
fn corner_radius_nonzero_renders_value() {
    let mut theme = minimal_theme();
    theme.picom = Some(picom(false, false, 12));
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("corner-radius = 12;"));
}

#[test]
fn corner_radius_zero_renders_zero() {
    let mut theme = minimal_theme();
    theme.picom = Some(picom(false, false, 0));
    let out = render(&template_path(), &theme).unwrap();
    assert!(out.contains("corner-radius = 0;"));
}

#[test]
fn corner_radius_defaults_to_zero_when_unset() {
    let out = render(&template_path(), &minimal_theme()).unwrap();
    assert!(out.contains("corner-radius = 0;"));
}
