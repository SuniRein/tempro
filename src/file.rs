use std::path::PathBuf;

/// Get the path to the template directory.
/// First, it checks if the `TEMPRO_HOME` environment variable is set.
/// If not, it checks for `XDG_CONFIG_HOME` or falls back to `~/.config/tempro`.
pub fn get_template_path() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("TEMPRO_HOME") {
        return Some(PathBuf::from(path));
    }

    let base = if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else {
        match std::env::var("HOME") {
            Ok(home) => PathBuf::from(home).join(".config"),
            Err(_) => return None,
        }
    };

    Some(base.join("tempro"))
}
