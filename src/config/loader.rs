use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::schema::AxonConfig;

pub fn load_config(path: &Path) -> Result<AxonConfig> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read config file: {}", path.display()))?;

    let config: AxonConfig = toml::from_str(&content).context("Invalid config format")?;

    Ok(config)
}

pub fn get_config() -> crate::config::schema::AxonConfig {
    // Fallback pentru compilare
    unimplemented!("Config must be loaded via load_config() first");
}
