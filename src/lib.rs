//! `axtc` — Arch/X theme changer.
//!
//! Reads a theme from a single TOML file, renders a [Tera] template for each
//! supported application, then writes the result to its config location —
//! backing up any existing file first.
//!
//! # Theme file structure
//!
//! Every field except `name` is optional. Omit an entire section to skip that
//! application — `axtc` silently skips any app whose section is absent.
//!
//! ```toml
//! name        = "my-theme"
//! description = "Dark blue"
//!
//! [global]
//! font = "FiraCode"          # fallback font for all apps
//!
//! [ansi.primary]
//! foreground = "#e0e0e0"
//! background = "#1a1a2e"
//!
//! [ansi.normal]
//! red  = "#e06c75"
//! blue = "#61afef"
//!
//! [alacritty]
//! font_size = 12.0
//! opacity   = 0.95
//!
//! [polybar]
//! position = "top"
//! height   = 24
//!
//! [picom]
//! blur          = true
//! corner_radius = 8
//! ```
//!
//! # Tera templates
//!
//! Templates live at `~/.config/axtc/templates/<app>/<file>.tera`. The entire
//! [`Theme`] is serialized into the Tera context, so every field is reachable
//! by its TOML path. Because all sections are optional, guard with
//! `{% if section %}` before accessing nested fields, and use
//! `| default(value=…)` for per-field fallbacks.
//!
//! ```text
//! {# Use ansi colors when the section is present #}
//! {% if ansi and ansi.primary %}
//! background = "{{ ansi.primary.background | default(value="#1a1a2e") }}"
//! foreground = "{{ ansi.primary.foreground | default(value="#e0e0e0") }}"
//! {% endif %}
//!
//! {# Fall back from app font → global font #}
//! {% if alacritty and alacritty.font %}
//! normal = { family = "{{ alacritty.font }}", style = "SemiBold" }
//! {% elif global and global.font %}
//! normal = { family = "{{ global.font }}", style = "SemiBold" }
//! {% endif %}
//!
//! size    = {{ alacritty.font_size | default(value=12.0) }}
//! opacity = {{ alacritty.opacity   | default(value=1.0)  }}
//! ```
//!
//! The full context key paths for each section are documented on the
//! corresponding config struct in the [`config`] module.
//!
//! [Tera]: https://keats.github.io/tera/
//! [`Theme`]: crate::theme::Theme
//! [`config`]: crate::theme::config

#![deny(missing_docs)]

pub mod apply;
pub mod template;
pub mod theme;
