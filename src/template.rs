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

    pub fn location(&self) -> &PathBuf {
        &self.location
    }

    const META_FILE: &'static str = "meta.toml";

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
