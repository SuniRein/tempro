use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};

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

/// Get all template names in the template home.
/// Only directories are considered.
/// It *does not* check if the template is valid.
pub fn get_all_template_names(home: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();

    for entry in home
        .read_dir()
        .with_context(|| format!("failed to read template home: {}", home.display()))?
    {
        let entry = entry.with_context(|| "failed to read a directory entry")?;
        if entry.path().is_dir() {
            let os_name = entry.file_name();
            let name = os_name
                .into_string()
                .map_err(|os| anyhow!("template name not valid UTF-8: {:?}", os))?;
            names.push(name);
        }
    }

    Ok(names)
}

pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        bail!("source path {} is not a directory", src.display());
    }

    if dst.exists() {
        bail!("destination path {} already exists", dst.display());
    }

    std::fs::create_dir_all(dst)
        .with_context(|| format!("failed to create directory: {}", dst.display()))?;

    for entry in src
        .read_dir()
        .with_context(|| format!("failed to read source directory: {}", src.display()))?
    {
        let entry = entry.with_context(|| "failed to read a directory entry")?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)
                .with_context(|| format!("failed to copy file: {}", src_path.display()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use hamcrest2::prelude::*;

    mod test_get_template_home {
        use super::*;

        use temp_env::with_vars;

        const TEMPRO_HOME: &str = "/custom/tempro/home";
        const XDG_CONFIG_HOME: &str = "/custom/xdg/config";
        const HOME: &str = "/custom/home";

        #[test]
        fn with_tempro_home() {
            with_vars(
                [
                    ("TEMPRO_HOME", Some(TEMPRO_HOME)),
                    ("XDG_CONFIG_HOME", Some(XDG_CONFIG_HOME)),
                    ("HOME", Some(HOME)),
                ],
                || {
                    assert_eq!(
                        get_template_home(),
                        Some(PathBuf::from("/custom/tempro/home"))
                    );
                },
            );
        }

        #[test]
        fn fallback_xdg_config_home() {
            with_vars(
                [
                    ("TEMPRO_HOME", None),
                    ("XDG_CONFIG_HOME", Some(XDG_CONFIG_HOME)),
                    ("HOME", Some(HOME)),
                ],
                || {
                    assert_eq!(
                        get_template_home(),
                        Some(PathBuf::from("/custom/xdg/config/tempro"))
                    );
                },
            );
        }

        #[test]
        fn fallback_home() {
            with_vars(
                [
                    ("TEMPRO_HOME", None),
                    ("XDG_CONFIG_HOME", None),
                    ("HOME", Some(HOME)),
                ],
                || {
                    assert_eq!(
                        get_template_home(),
                        Some(PathBuf::from("/custom/home/.config/tempro"))
                    );
                },
            );
        }

        #[test]
        fn fail_to_determine() {
            with_vars(
                [
                    ("TEMPRO_HOME", None::<&str>),
                    ("XDG_CONFIG_HOME", None),
                    ("HOME", None),
                ],
                || {
                    assert_eq!(get_template_home(), None);
                },
            );
        }
    }

    mod test_get_all_template_names {
        use super::*;

        use std::fs;

        #[test]
        fn empty_dir() {
            let temp_dir = tempfile::tempdir().unwrap();
            let home = temp_dir.path();

            let result = get_all_template_names(home).unwrap();

            assert_that!(&result, empty());
        }

        #[test]
        fn work_correctly() {
            let temp_dir = tempfile::tempdir().unwrap();
            let home = temp_dir.path();

            fs::create_dir(home.join("template1")).unwrap();
            fs::create_dir(home.join("template2")).unwrap();
            fs::File::create(home.join("template_ignored")).unwrap();

            let result = get_all_template_names(home).unwrap();

            assert_that!(&result, len(2));
            assert_that!(
                &result,
                contains(vec!["template1".to_string(), "template2".to_string()])
            );
            assert_that!(&result, not(contains("template_ignored".to_string())));
        }

        #[test]
        fn invalid_home() {
            let result = get_all_template_names(Path::new("/invalid/path"));
            assert_that!(result, err());
        }
    }
}
