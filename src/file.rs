use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

/// Get the path to the template home.
/// First, it checks if the `TEMPRO_HOME` environment variable is set.
/// If not, it checks for `XDG_CONFIG_HOME` or falls back to `~/.config/tempro`.
/// It *does not* check if the path exists.
pub fn get_template_home() -> Option<PathBuf> {
    if let Ok(path) = env::var("TEMPRO_HOME") {
        return Some(PathBuf::from(path));
    }

    let base = if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else {
        match env::var("HOME") {
            Ok(home) => PathBuf::from(home).join(".config"),
            Err(_) => return None,
        }
    };

    Some(base.join("tempro"))
}

/// Get all template paths in the template home.
/// Only directories are considered.
/// It *does not* check if the template is valid.
pub fn get_all_template_paths(home: &Path) -> Result<Vec<PathBuf>> {
    let result = home
        .read_dir()
        .with_context(|| format!("failed to read template home: {}", home.display()))?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() { Some(path) } else { None }
        })
        .collect();

    Ok(result)
}
