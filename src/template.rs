//! Tera template rendering.

use anyhow::{Context, Result};
use std::path::Path;
use tera::{Context as TeraContext, Tera};

use crate::theme::Theme;

/// Render a Tera template file against a [`Theme`], returning the result as a [`String`].
///
/// The template is read from disk each call; no caching is performed.
pub fn render(template_path: &Path, theme: &Theme) -> Result<String> {
    let template_str = std::fs::read_to_string(template_path)
        .with_context(|| format!("could not read template '{}'", template_path.display()))?;

    let mut tera = Tera::default();
    tera.add_raw_template("t", &template_str)
        .context("could not parse template")?;

    let context = TeraContext::from_serialize(theme).context("could not build template context")?;

    tera.render("t", &context)
        .context("template rendering failed")
}
