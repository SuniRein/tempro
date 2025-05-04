use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::template::Template;

#[derive(Debug)]
pub struct TemplateHome {
    home: TempDir,
    dirs: Vec<TemplateDir>,
}

impl TemplateHome {
    pub fn new() -> Self {
        let home = tempfile::tempdir().expect("failed to create temp dir");
        Self { home, dirs: vec![] }
    }

    pub fn single(name: &str, content: Option<&str>) -> Self {
        let mut new = Self::new();
        new.push(name, content);
        new
    }

    pub fn push(&mut self, name: &str, content: Option<&str>) {
        let template_dir = TemplateDir::new(self.home.path(), name, content);
        self.dirs.push(template_dir);
    }

    pub fn path(&self) -> &Path {
        self.home.path()
    }

    pub fn dirs(&self) -> &[TemplateDir] {
        &self.dirs
    }
}

#[derive(Debug, Clone)]
pub struct TemplateDir {
    name: String,
    path: PathBuf,
}

impl TemplateDir {
    fn new(home: &Path, name: &str, content: Option<&str>) -> Self {
        let template_dir = home.join(name);
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
            name: name.into(),
            path: template_dir,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
