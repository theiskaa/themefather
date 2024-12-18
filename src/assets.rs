//! Assets module for managing embedded theme templates
//!
//! This module handles loading and accessing embedded theme template files for different platforms.
//! It provides a static mapping of platform names to template file paths and functionality to
//! retrieve template contents as UTF-8 strings.
//!
//! The templates are stored in the assets/themes directory and are embedded into the binary
//! at compile time using rust-embed.
//!

#![allow(dead_code)]

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use std::collections::HashMap;

/// Container for embedded resource files like fonts. Uses the assets/ directory
/// as the root for embedded files.
#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// Static mapping that associates font names with their corresponding embedded file paths.
/// Font names are stored in lowercase for case-insensitive lookups.
static TEMPLATE_THEMES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("android", "themes/template_android_theme.txt");
    m.insert("ios", "themes/template_ios_theme.txt");
    m.insert("macos", "themes/template_macos_theme.txt");
    m.insert("tdesktop", "themes/template_tdesktop_theme.txt");
    m
});

/// Retrieves the content of an embedded text asset as a UTF-8 string.
/// Returns None if the asset doesn't exist or cannot be decoded as UTF-8.
pub fn get_theme_template(platform: &str) -> Option<String> {
    if let Some(path) = TEMPLATE_THEMES.get(platform.to_lowercase().as_str()) {
        Assets::get(path).and_then(|f| String::from_utf8(f.data.to_vec()).ok())
    } else {
        None
    }
}
