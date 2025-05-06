use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use super::Template;

impl Template {
    pub fn apply(&self, path: &Path) -> Result<()> {
        let template_dir = self.location().join(Self::TEMPLATE_DIR);
        copy_dir(&template_dir, path)
    }
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if !src.is_dir() {
        bail!("source path {} is not a directory", src.display());
    }

    if dst.exists() {
        bail!("destination path {} already exists", dst.display());
    }

    fs::create_dir_all(dst)
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

    use std::fs;

    use hamcrest2::prelude::*;

    use crate::test_utils::TemplateHome;

    fn setup() -> (TemplateHome, Template) {
        let home = TemplateHome::single("test template", Some(r#"description = "Test""#));

        let template_path = home.dirs()[0].path();

        let template_dir = template_path.join(Template::TEMPLATE_DIR);
        fs::create_dir(&template_dir).unwrap();
        fs::write(template_dir.join("file.txt"), "Some content").unwrap();
        fs::write(template_dir.join("another_file.txt"), "Ohter content").unwrap();

        let template = Template::load(template_path).unwrap();

        (home, template)
    }

    #[test]
    fn it_works() {
        let (_home, template) = setup();

        let temp_dir = tempfile::tempdir().unwrap();
        let target_path = temp_dir.path().join("target");
        template.apply(&target_path).unwrap();

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

    // TODO:implement the following test
    #[test]
    fn template_dir_missing() {}

    // TODO:implement the following test
    #[test]
    fn template_dir_is_file() {}

    // TODO:implement the following test
    #[test]
    fn target_path_is_file() {}

    // TODO:implement the following test
    #[test]
    fn target_path_already_exists() {}
}
