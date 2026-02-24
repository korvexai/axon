use anyhow::{Result, Context};
use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Save data safely using atomic write:
/// 1. Write to temp file
/// 2. Rename to final file
pub fn save_to_file<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_path = temp_path(path);

    let json = serde_json::to_string_pretty(data)
        .context("Failed to serialize JSON")?;

    {
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?; // force flush to disk
    }

    fs::rename(&temp_path, path)
        .context("Atomic rename failed")?;

    Ok(())
}

/// Load JSON file into struct
pub fn load_from_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let content = fs::read_to_string(path)
        .context("Failed to read file")?;

    let data = serde_json::from_str(&content)
        .context("Failed to deserialize JSON")?;

    Ok(data)
}

/// Check if file exists
pub fn exists(path: &Path) -> bool {
    path.exists()
}

/// Delete file safely
pub fn delete(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Create temporary path for atomic write
fn temp_path(original: &Path) -> PathBuf {
    let mut tmp = original.to_path_buf();
    tmp.set_extension("tmp");
    tmp
}



