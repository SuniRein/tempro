use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::file;

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
    pub const TEMPLATE_DIR: &'static str = "template";

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

    pub fn write_to(&self, path: &Path) -> Result<()> {
        let template_dir = self.location().join(Self::TEMPLATE_DIR);
        file::copy_dir(&template_dir, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hamcrest2::prelude::*;

    use crate::test_utils::TemplateHome;

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
            let home = TemplateHome::single("test template", None);
            assert_that!(Template::read_from_path(home.dirs()[0].path()), err());
        }

        #[test]
        fn meta_file_is_invalid_toml() {
            let home = TemplateHome::single("test template", Some("invalid toml content"));
            assert_that!(Template::read_from_path(home.dirs()[0].path()), err());
        }

        #[test]
        fn meta_file_missing_description_field() {
            let home = TemplateHome::single("test template", Some(r#"title = "no description""#));
            assert_that!(Template::read_from_path(home.dirs()[0].path()), err());
        }

        #[test]
        fn valid_template() {
            let home = TemplateHome::single("test template", Some(r#"description = "Test""#));
            let dir = &home.dirs()[0];
            let template = Template::read_from_path(dir.path()).unwrap();

            assert_eq!(template.name(), dir.name());
            assert_eq!(template.description(), "Test");
            assert_eq!(template.location(), dir.path());
        }
    }

    mod test_write_to {
        use super::*;

        fn setup() -> (TemplateHome, Template) {
            let home = TemplateHome::single("test template", Some(r#"description = "Test""#));

            let template_path = home.dirs()[0].path();

            let template_dir = template_path.join(Template::TEMPLATE_DIR);
            fs::create_dir(&template_dir).unwrap();
            fs::write(template_dir.join("file.txt"), "Some content").unwrap();
            fs::write(template_dir.join("another_file.txt"), "Ohter content").unwrap();

            let template = Template::read_from_path(template_path).unwrap();

            (home, template)
        }

        #[test]
        fn it_works() {
            let (_home, template) = setup();

            let temp_dir = tempfile::tempdir().unwrap();
            let target_path = temp_dir.path().join("target");
            template.write_to(&target_path).unwrap();

            assert_that!(&target_path, path_exists());
            assert_that!(&target_path.join("file.txt"), file_exists());
            assert_that!(&target_path.join("another_file.txt"), file_exists());

            assert_eq!(
                fs::read_to_string(target_path.join("file.txt")).unwrap(),
                "Some content"
            );
            assert_eq!(
                fs::read_to_string(target_path.join("another_file.txt")).unwrap(),
                "Ohter content"
            );
        }

        #[test]
        fn template_dir_missing() {}

        #[test]
        fn template_dir_is_file() {}

        #[test]
        fn target_path_is_file() {}

        #[test]
        fn target_path_already_exists() {}
    }
}
