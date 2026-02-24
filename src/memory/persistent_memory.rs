use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct DiskMemory {
    pub conversation_id: Uuid,
    pub entries: Vec<String>,
}

pub fn save_memory(path: PathBuf, memory: &DiskMemory) {
    if let Ok(json) = serde_json::to_string_pretty(memory) {
        let _ = fs::write(path, json);
    }
}

pub fn load_memory(path: PathBuf) -> Option<DiskMemory> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}






