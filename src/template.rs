use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Template {
    name: String,
    location: PathBuf,
    meta: Meta,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    description: String,
}

impl Template {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.meta.description
    }

    pub fn location(&self) -> &Path {
        &self.location
    }

    pub const META_FILE: &'static str = "meta.toml";

    pub fn read_from_path(path: &Path) -> Result<Self> {
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

    use hamcrest2::prelude::*;

    use crate::test_utils::TemplateDir;

    mod test_read_from_path {
        use super::*;

        #[test]
        fn path_invalid() {
            assert_that!(
                Template::read_from_path(&PathBuf::from("/invalid/path/..")),
                err()
            );
        }

        #[test]
        fn meta_file_missing() {
            let template_dir = TemplateDir::new("test template", None);
            assert_that!(Template::read_from_path(template_dir.path()), err());
        }

        #[test]
        fn meta_file_is_invalid_toml() {
            let template_dir = TemplateDir::new("test template", Some("invalid toml content"));
            assert_that!(Template::read_from_path(template_dir.path()), err());
        }

        #[test]
        fn meta_file_missing_description_field() {
            let template_dir =
                TemplateDir::new("test template", Some(r#"title = "no description""#));
            assert_that!(Template::read_from_path(template_dir.path()), err());
        }

        #[test]
        fn valid_template() {
            let template_dir =
                TemplateDir::new("test template", Some(r#"description = "Test template""#));

            let template = Template::read_from_path(template_dir.path()).unwrap();

            assert_eq!(template.name(), template_dir.name());
            assert_eq!(template.description(), "Test template");
            assert_eq!(template.location(), template_dir.path());
        }
    }
}
