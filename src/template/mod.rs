pub mod apply;
pub mod load;
mod meta;

use std::path::{Path, PathBuf};

use meta::Meta;

#[derive(Debug)]
pub struct Template {
    name: String,
    location: PathBuf,
    meta: Meta,
}

impl Template {
    pub const META_FILE: &'static str = "meta.toml";
    pub const TEMPLATE_DIR: &'static str = "template";

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.meta.description
    }

    pub fn location(&self) -> &Path {
        &self.location
    }
}
