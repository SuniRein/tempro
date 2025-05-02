use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Template {
    meta: Meta,
    location: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    name: String,
    description: String,
}

impl Template {
    pub fn name(&self) -> &str {
        &self.meta.name
    }

    pub fn description(&self) -> &str {
        &self.meta.description
    }

    pub fn location(&self) -> &PathBuf {
        &self.location
    }

    const META_FILE: &'static str = "meta.toml";

    pub fn read_from_path(path: &PathBuf) -> Result<Self> {
        let meta_file = path.join(Self::META_FILE);

        let meta_content = fs::read_to_string(&meta_file)
            .with_context(|| format!("failed to read meta file: {}", meta_file.display()))?;

        let meta = toml::from_str(&meta_content)
            .with_context(|| format!("failed to parse meta file: {}", meta_file.display()))?;

        Ok(Template {
            meta,
            location: path.to_path_buf(),
        })
    }
}
