use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_threshold_mins: u64,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub timeout: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_threshold_mins: 5,
            start_time: None,
            end_time: None,
            timeout: None,
        }
    }
}

pub fn load_config() -> Result<Config> {
    let mut path =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    path.push(".neflo");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    path.push("config.json");

    if !path.exists() {
        let config = Config::default();
        let data = serde_json::to_string_pretty(&config)?;
        fs::write(&path, data)?;
        return Ok(config);
    }

    let data = fs::read_to_string(&path)?;
    let config = serde_json::from_str(&data)?;
    Ok(config)
}
