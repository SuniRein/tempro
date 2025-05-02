use std::path::PathBuf;

#[derive(Debug)]
pub struct Template {
    name: String,
    description: String,
    location: PathBuf,
}

impl Template {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn location(&self) -> &PathBuf {
        &self.location
    }
}
