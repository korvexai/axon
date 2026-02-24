use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub struct VectorStore {
    pub data: HashMap<String, Vec<(String, Vec<f32>)>>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file: String, chunk: String, embedding: Vec<f32>) {
        self.data.entry(file).or_default().push((chunk, embedding)); // Ã¢Å“â€¦ parantezÃ„Æ’ ÃƒÂ®nchisÃ„Æ’ corect
    }

    pub fn save(&self, _path: &Path) -> Result<()> {
        Ok(())
    }
}

impl Default for VectorStore { fn default() -> Self { Self::new() } }
