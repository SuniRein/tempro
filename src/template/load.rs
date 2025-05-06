use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use super::Template;

impl Template {
    pub fn load(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("failed to get template name from path"))?
            .to_string();

        let meta_file = path.join(Self::META_FILE);

        let meta_content = fs::read_to_string(&meta_file)
            .with_context(|| format!("failed to read meta file: {}", meta_file.display()))?;

        let meta = toml::from_str(&meta_content)
            .with_context(|| format!("failed to parse meta file: {}", meta_file.display()))?;

        Ok(Template {
            name,
            location: path.to_path_buf(),
            meta,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use hamcrest2::prelude::*;

    use crate::test_utils::TemplateHome;

    #[test]
    fn path_invalid() {
        assert_that!(Template::load(&PathBuf::from("/invalid/path/..")), err());
    }

    #[test]
    fn meta_file_missing() {
        let home = TemplateHome::single("test template", None);
        assert_that!(Template::load(home.dirs()[0].path()), err());
    }

    #[test]
    fn meta_file_is_invalid_toml() {
        let home = TemplateHome::single("test template", Some("invalid toml content"));
        assert_that!(Template::load(home.dirs()[0].path()), err());
    }

    #[test]
    fn meta_file_missing_description_field() {
        let home = TemplateHome::single("test template", Some(r#"title = "no description""#));
        assert_that!(Template::load(home.dirs()[0].path()), err());
    }

    #[test]
    fn valid_template() {
        let home = TemplateHome::single("test template", Some(r#"description = "Test""#));
        let dir = &home.dirs()[0];
        let template = Template::load(dir.path()).unwrap();

        assert_eq!(template.name(), dir.name());
        assert_eq!(template.description(), "Test");
        assert_eq!(template.location(), dir.path());
    }
}
