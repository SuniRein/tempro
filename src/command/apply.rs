use std::env;
use std::path::Path;

use anyhow::Result;

use crate::cli::ApplyArgs;
use crate::template::Template;

pub fn handle_apply_command(home: &Path, args: &ApplyArgs) -> Result<()> {
    let current_dir = env::current_dir()?;
    let target_dir = current_dir.join(&args.target);

    let template = Template::load(&home.join(&args.name))?;
    template.apply(&target_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use crate::test_utils::TemplateHome;
    use crate::test_utils::prelude::*;

    fn setup_home() -> TemplateHome {
        let home = TemplateHome::single("test template", Some(r#"description = "Test template""#));
        let template_path = &home.dirs()[0];

        let template_dir = template_path.path().join("template");
        fs::create_dir(&template_dir).unwrap();
        fs::write(template_dir.join("file1"), "some content1").unwrap();
        fs::write(template_dir.join("file2"), "some content2").unwrap();
        fs::create_dir(template_dir.join("dir")).unwrap();
        fs::write(template_dir.join("dir/file3"), "some content3").unwrap();

        home
    }

    fn setup_target() -> (TempDir, PathBuf) {
        let temp_dir = tempfile::tempdir().unwrap();
        let target = temp_dir.path().join("target");
        (temp_dir, target)
    }

    #[gtest]
    fn it_works() {
        let home = setup_home();
        let (temp_dir, target) = setup_target();

        env::set_current_dir(temp_dir.path()).unwrap();

        let args = ApplyArgs {
            name: "test template".to_string(),
            target: "target".to_string(),
        };
        handle_apply_command(home.path(), &args).unwrap();

        assert_that!(target, dir_exist());
        expect_that!(target.join("file1"), file("some content1"));
        expect_that!(target.join("file2"), file("some content2"));
        assert_that!(target.join("dir"), dir_exist());
        expect_that!(target.join("dir/file3"), file("some content3"));
    }
}
