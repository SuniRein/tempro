use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::template::Template;

pub struct TemplateDir {
    _temp_dir: TempDir,
    name: String,
    template_dir: PathBuf,
}

impl TemplateDir {
    pub fn new(name: &str, content: Option<&str>) -> Self {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");

        let template_dir = temp_dir.path().join(name);
        fs::create_dir(&template_dir).unwrap_or_else(|err| {
            panic!(
                "failed to create template dir {}: {err}",
                template_dir.display()
            )
        });

        if let Some(content) = content {
            let meta_file = template_dir.join(Template::META_FILE);
            fs::write(&meta_file, content).unwrap_or_else(|err| {
                panic!("failed to write meta file {}: {err}", meta_file.display())
            });
        }

        Self {
            _temp_dir: temp_dir,
            name: name.to_string(),
            template_dir,
        }
    }

    pub fn path(&self) -> &Path {
        &self.template_dir
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
