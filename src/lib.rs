//! `axtc` — Arch/X theme changer.
//!
//! Renders Tera templates for herbstluftwm, polybar, alacritty, and picom
//! from a single TOML theme file, then writes the results to their respective
//! config locations.

#![deny(missing_docs)]

pub mod apply;
pub mod template;
pub mod theme;
